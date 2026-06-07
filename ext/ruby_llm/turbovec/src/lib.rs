use magnus::{exception, function, method, prelude::*, Error, Ruby};
use rb_sys as _;
use std::path::Path;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use turbovec::{
    AddError, ConstructError, IdMapIndex as NativeIdMapIndex,
    SearchResults as NativeSearchResults, TurboQuantIndex as NativeTurboQuantIndex,
};

fn construct_error(err: ConstructError) -> Error {
    Error::new(exception::arg_error(), err.to_string())
}

fn add_error(err: AddError) -> Error {
    let class = match err {
        AddError::IdAlreadyPresent(_) => exception::key_error(),
        _ => exception::arg_error(),
    };
    Error::new(class, err.to_string())
}

fn io_error(err: std::io::Error) -> Error {
    Error::new(exception::runtime_error(), err.to_string())
}

fn lock_read<'a, T>(lock: &'a RwLock<T>) -> Result<RwLockReadGuard<'a, T>, Error> {
    lock.read()
        .map_err(|_| Error::new(exception::runtime_error(), "index lock poisoned"))
}

fn lock_write<'a, T>(lock: &'a RwLock<T>) -> Result<RwLockWriteGuard<'a, T>, Error> {
    lock.write()
        .map_err(|_| Error::new(exception::runtime_error(), "index lock poisoned"))
}

fn validate_queries(queries: &[f32], dim: Option<usize>, kind: &str) -> Result<(), Error> {
    if let Some(dim) = dim {
        if dim != 0 && queries.len() % dim != 0 {
            return Err(Error::new(
                exception::arg_error(),
                format!(
                    "{kind} buffer length {} is not a multiple of dim {}",
                    queries.len(),
                    dim
                ),
            ));
        }
    }

    Ok(())
}

fn query_range(qi: usize, nq: usize, k: usize) -> Result<std::ops::Range<usize>, Error> {
    if qi >= nq {
        return Err(Error::new(
            exception::arg_error(),
            format!("query index {qi} out of bounds for {nq} queries"),
        ));
    }

    let start = qi * k;
    Ok(start..start + k)
}

#[magnus::wrap(class = "RubyLLM::Turbovec::SearchResults", free_immediately)]
struct RubySearchResults {
    scores: Vec<f32>,
    indices: Vec<i64>,
    nq: usize,
    k: usize,
}

impl From<NativeSearchResults> for RubySearchResults {
    fn from(results: NativeSearchResults) -> Self {
        Self {
            scores: results.scores,
            indices: results.indices,
            nq: results.nq,
            k: results.k,
        }
    }
}

impl RubySearchResults {
    fn scores(&self) -> Vec<f32> {
        self.scores.clone()
    }

    fn indices(&self) -> Vec<i64> {
        self.indices.clone()
    }

    fn nq(&self) -> usize {
        self.nq
    }

    fn k(&self) -> usize {
        self.k
    }

    fn scores_for_query(&self, qi: usize) -> Result<Vec<f32>, Error> {
        let range = query_range(qi, self.nq, self.k)?;
        Ok(self.scores[range].to_vec())
    }

    fn indices_for_query(&self, qi: usize) -> Result<Vec<i64>, Error> {
        let range = query_range(qi, self.nq, self.k)?;
        Ok(self.indices[range].to_vec())
    }
}

#[magnus::wrap(class = "RubyLLM::Turbovec::TurboQuantIndex", free_immediately)]
struct RubyTurboQuantIndex {
    inner: RwLock<NativeTurboQuantIndex>,
}

impl RubyTurboQuantIndex {
    fn new(dim: usize, bit_width: usize) -> Result<Self, Error> {
        NativeTurboQuantIndex::new(dim, bit_width)
            .map(|inner| Self {
                inner: RwLock::new(inner),
            })
            .map_err(construct_error)
    }

    fn new_lazy(bit_width: usize) -> Result<Self, Error> {
        NativeTurboQuantIndex::new_lazy(bit_width)
            .map(|inner| Self {
                inner: RwLock::new(inner),
            })
            .map_err(construct_error)
    }

    fn load(path: String) -> Result<Self, Error> {
        NativeTurboQuantIndex::load(Path::new(&path))
            .map(|inner| Self {
                inner: RwLock::new(inner),
            })
            .map_err(io_error)
    }

    fn add(&self, vectors: Vec<f32>) -> Result<(), Error> {
        let mut inner = lock_write(&self.inner)?;
        let dim = inner.dim_opt().ok_or_else(|| {
            Error::new(
                exception::arg_error(),
                "index dimension is not set; use add_with_dim on lazy indexes",
            )
        })?;

        inner.add_2d(&vectors, dim).map_err(add_error)
    }

    fn add_with_dim(&self, vectors: Vec<f32>, dim: usize) -> Result<(), Error> {
        lock_write(&self.inner)?.add_2d(&vectors, dim).map_err(add_error)
    }

    fn search(&self, queries: Vec<f32>, k: usize) -> Result<RubySearchResults, Error> {
        let inner = lock_read(&self.inner)?;
        validate_queries(&queries, inner.dim_opt(), "query")?;
        Ok(inner.search(&queries, k).into())
    }

    fn search_with_mask(
        &self,
        queries: Vec<f32>,
        k: usize,
        mask: Option<Vec<bool>>,
    ) -> Result<RubySearchResults, Error> {
        let inner = lock_read(&self.inner)?;
        validate_queries(&queries, inner.dim_opt(), "query")?;

        if let Some(ref mask) = mask {
            if mask.len() != inner.len() {
                return Err(Error::new(
                    exception::arg_error(),
                    format!(
                        "mask length {} does not match index size {}",
                        mask.len(),
                        inner.len()
                    ),
                ));
            }
        }

        Ok(inner.search_with_mask(&queries, k, mask.as_deref()).into())
    }

    fn prepare(&self) -> Result<(), Error> {
        lock_read(&self.inner)?.prepare();
        Ok(())
    }

    fn write(&self, path: String) -> Result<(), Error> {
        lock_read(&self.inner)?
            .write(Path::new(&path))
            .map_err(io_error)
    }

    fn len(&self) -> Result<usize, Error> {
        Ok(lock_read(&self.inner)?.len())
    }

    fn is_empty(&self) -> Result<bool, Error> {
        Ok(lock_read(&self.inner)?.is_empty())
    }

    fn dim(&self) -> Result<usize, Error> {
        Ok(lock_read(&self.inner)?.dim())
    }

    fn dim_opt(&self) -> Result<Option<usize>, Error> {
        Ok(lock_read(&self.inner)?.dim_opt())
    }

    fn bit_width(&self) -> Result<usize, Error> {
        Ok(lock_read(&self.inner)?.bit_width())
    }

    fn swap_remove(&self, idx: usize) -> Result<usize, Error> {
        Ok(lock_write(&self.inner)?.swap_remove(idx))
    }
}

#[magnus::wrap(class = "RubyLLM::Turbovec::IdMapIndex", free_immediately)]
struct RubyIdMapIndex {
    inner: RwLock<NativeIdMapIndex>,
}

impl RubyIdMapIndex {
    fn new(dim: usize, bit_width: usize) -> Result<Self, Error> {
        NativeIdMapIndex::new(dim, bit_width)
            .map(|inner| Self {
                inner: RwLock::new(inner),
            })
            .map_err(construct_error)
    }

    fn new_lazy(bit_width: usize) -> Result<Self, Error> {
        NativeIdMapIndex::new_lazy(bit_width)
            .map(|inner| Self {
                inner: RwLock::new(inner),
            })
            .map_err(construct_error)
    }

    fn load(path: String) -> Result<Self, Error> {
        NativeIdMapIndex::load(Path::new(&path))
            .map(|inner| Self {
                inner: RwLock::new(inner),
            })
            .map_err(io_error)
    }

    fn add_with_ids(&self, vectors: Vec<f32>, ids: Vec<u64>) -> Result<(), Error> {
        let mut inner = lock_write(&self.inner)?;
        if inner.dim_opt().is_none() {
            return Err(Error::new(
                exception::arg_error(),
                "index dimension is not set; use add_with_ids_2d on lazy indexes",
            ));
        }

        inner.add_with_ids(&vectors, &ids).map_err(add_error)
    }

    fn add_with_ids_2d(&self, vectors: Vec<f32>, dim: usize, ids: Vec<u64>) -> Result<(), Error> {
        lock_write(&self.inner)?
            .add_with_ids_2d(&vectors, dim, &ids)
            .map_err(add_error)
    }

    fn remove(&self, id: u64) -> Result<bool, Error> {
        Ok(lock_write(&self.inner)?.remove(id))
    }

    fn search(&self, queries: Vec<f32>, k: usize) -> Result<(Vec<f32>, Vec<u64>), Error> {
        let inner = lock_read(&self.inner)?;
        validate_queries(&queries, inner.dim_opt(), "query")?;
        Ok(inner.search(&queries, k))
    }

    fn search_with_allowlist(
        &self,
        queries: Vec<f32>,
        k: usize,
        allowlist: Option<Vec<u64>>,
    ) -> Result<(Vec<f32>, Vec<u64>), Error> {
        let inner = lock_read(&self.inner)?;
        validate_queries(&queries, inner.dim_opt(), "query")?;

        if let Some(ref ids) = allowlist {
            if ids.is_empty() {
                return Err(Error::new(exception::arg_error(), "allowlist is empty"));
            }

            for id in ids {
                if !inner.contains(*id) {
                    return Err(Error::new(
                        exception::key_error(),
                        format!("id {id} in allowlist is not present in index"),
                    ));
                }
            }
        }

        Ok(inner.search_with_allowlist(&queries, k, allowlist.as_deref()))
    }

    fn contains(&self, id: u64) -> Result<bool, Error> {
        Ok(lock_read(&self.inner)?.contains(id))
    }

    fn prepare(&self) -> Result<(), Error> {
        lock_read(&self.inner)?.prepare();
        Ok(())
    }

    fn write(&self, path: String) -> Result<(), Error> {
        lock_read(&self.inner)?
            .write(Path::new(&path))
            .map_err(io_error)
    }

    fn len(&self) -> Result<usize, Error> {
        Ok(lock_read(&self.inner)?.len())
    }

    fn is_empty(&self) -> Result<bool, Error> {
        Ok(lock_read(&self.inner)?.is_empty())
    }

    fn dim(&self) -> Result<usize, Error> {
        Ok(lock_read(&self.inner)?.dim())
    }

    fn dim_opt(&self) -> Result<Option<usize>, Error> {
        Ok(lock_read(&self.inner)?.dim_opt())
    }

    fn bit_width(&self) -> Result<usize, Error> {
        Ok(lock_read(&self.inner)?.bit_width())
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let ruby_llm = ruby.define_module("RubyLLM")?;
    let turbovec = ruby_llm.define_module("Turbovec")?;

    let index_class = turbovec.define_class("TurboQuantIndex", ruby.class_object())?;
    index_class.undef_default_alloc_func();
    index_class.define_singleton_method("new", function!(RubyTurboQuantIndex::new, 2))?;
    index_class.define_singleton_method("new_lazy", function!(RubyTurboQuantIndex::new_lazy, 1))?;
    index_class.define_singleton_method("load", function!(RubyTurboQuantIndex::load, 1))?;
    index_class.define_method("add", method!(RubyTurboQuantIndex::add, 1))?;
    index_class.define_method("add_with_dim", method!(RubyTurboQuantIndex::add_with_dim, 2))?;
    index_class.define_method("search", method!(RubyTurboQuantIndex::search, 2))?;
    index_class.define_method("search_with_mask", method!(RubyTurboQuantIndex::search_with_mask, 3))?;
    index_class.define_method("prepare", method!(RubyTurboQuantIndex::prepare, 0))?;
    index_class.define_method("write", method!(RubyTurboQuantIndex::write, 1))?;
    index_class.define_method("len", method!(RubyTurboQuantIndex::len, 0))?;
    index_class.define_method("empty?", method!(RubyTurboQuantIndex::is_empty, 0))?;
    index_class.define_method("dim", method!(RubyTurboQuantIndex::dim, 0))?;
    index_class.define_method("dim_opt", method!(RubyTurboQuantIndex::dim_opt, 0))?;
    index_class.define_method("bit_width", method!(RubyTurboQuantIndex::bit_width, 0))?;
    index_class.define_method("swap_remove", method!(RubyTurboQuantIndex::swap_remove, 1))?;

    let results_class = turbovec.define_class("SearchResults", ruby.class_object())?;
    results_class.undef_default_alloc_func();
    results_class.define_method("scores", method!(RubySearchResults::scores, 0))?;
    results_class.define_method("indices", method!(RubySearchResults::indices, 0))?;
    results_class.define_method("nq", method!(RubySearchResults::nq, 0))?;
    results_class.define_method("k", method!(RubySearchResults::k, 0))?;
    results_class.define_method("scores_for_query", method!(RubySearchResults::scores_for_query, 1))?;
    results_class.define_method("indices_for_query", method!(RubySearchResults::indices_for_query, 1))?;

    let id_map_class = turbovec.define_class("IdMapIndex", ruby.class_object())?;
    id_map_class.undef_default_alloc_func();
    id_map_class.define_singleton_method("new", function!(RubyIdMapIndex::new, 2))?;
    id_map_class.define_singleton_method("new_lazy", function!(RubyIdMapIndex::new_lazy, 1))?;
    id_map_class.define_singleton_method("load", function!(RubyIdMapIndex::load, 1))?;
    id_map_class.define_method("add_with_ids", method!(RubyIdMapIndex::add_with_ids, 2))?;
    id_map_class.define_method("add_with_ids_2d", method!(RubyIdMapIndex::add_with_ids_2d, 3))?;
    id_map_class.define_method("remove", method!(RubyIdMapIndex::remove, 1))?;
    id_map_class.define_method("search", method!(RubyIdMapIndex::search, 2))?;
    id_map_class.define_method("search_with_allowlist", method!(RubyIdMapIndex::search_with_allowlist, 3))?;
    id_map_class.define_method("contains?", method!(RubyIdMapIndex::contains, 1))?;
    id_map_class.define_method("prepare", method!(RubyIdMapIndex::prepare, 0))?;
    id_map_class.define_method("write", method!(RubyIdMapIndex::write, 1))?;
    id_map_class.define_method("len", method!(RubyIdMapIndex::len, 0))?;
    id_map_class.define_method("empty?", method!(RubyIdMapIndex::is_empty, 0))?;
    id_map_class.define_method("dim", method!(RubyIdMapIndex::dim, 0))?;
    id_map_class.define_method("dim_opt", method!(RubyIdMapIndex::dim_opt, 0))?;
    id_map_class.define_method("bit_width", method!(RubyIdMapIndex::bit_width, 0))?;

    Ok(())
}

