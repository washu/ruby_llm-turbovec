# Turbovec API Coverage Matrix

This matrix compares the public Rust crate surface against the Ruby gem wrapper.

## Ruby-exposed API coverage

| Turbovec Rust item | Ruby wrapper | Status | Notes |
| --- | --- | --- | --- |
| `TurboQuantIndex::new` | `RubyLLM::Turbovec::TurboQuantIndex.new` | Covered | Eager constructor |
| `TurboQuantIndex::new_lazy` | `RubyLLM::Turbovec::TurboQuantIndex.new_lazy` | Covered | Lazy constructor |
| `TurboQuantIndex::load` | `RubyLLM::Turbovec::TurboQuantIndex.load` | Covered | Loads `.tv` files |
| `TurboQuantIndex::add` / `add_2d` | `#add`, `#add_with_dim` | Covered | Ruby uses `Vec<f32>` input |
| `TurboQuantIndex::search` | `#search` | Covered | Returns `SearchResults` wrapper |
| `TurboQuantIndex::search_with_mask` | `#search_with_mask` | Covered | Ruby mask is `Array<Boolean>` |
| `TurboQuantIndex::prepare` | `#prepare` | Covered | Eager cache warm-up |
| `TurboQuantIndex::write` | `#write` | Covered | Serializes `.tv` files |
| `TurboQuantIndex::swap_remove` | `#swap_remove` | Covered | Exposed as a direct Ruby method |
| `TurboQuantIndex::len` | `#len` | Covered | Metadata |
| `TurboQuantIndex::is_empty` | `#empty?` | Covered | Metadata |
| `TurboQuantIndex::dim` / `dim_opt` | `#dim`, `#dim_opt` | Covered | Metadata |
| `TurboQuantIndex::bit_width` | `#bit_width` | Covered | Metadata |
| `SearchResults::scores` | `#scores` | Covered | Returns a copied Ruby array |
| `SearchResults::indices` | `#indices` | Covered | Returns a copied Ruby array |
| `SearchResults::nq` | `#nq` | Covered | Query count |
| `SearchResults::k` | `#k` | Covered | Effective k |
| `SearchResults::scores_for_query` | `#scores_for_query` | Covered | Per-query slice |
| `SearchResults::indices_for_query` | `#indices_for_query` | Covered | Per-query slice |
| `IdMapIndex::new` | `RubyLLM::Turbovec::IdMapIndex.new` | Covered | Stable-ID eager constructor |
| `IdMapIndex::new_lazy` | `RubyLLM::Turbovec::IdMapIndex.new_lazy` | Covered | Stable-ID lazy constructor |
| `IdMapIndex::load` | `RubyLLM::Turbovec::IdMapIndex.load` | Covered | Loads `.tvim` files |
| `IdMapIndex::add_with_ids` | `#add_with_ids` | Covered | Stable IDs with known dim |
| `IdMapIndex::add_with_ids_2d` | `#add_with_ids_2d` | Covered | Stable IDs with explicit dim |
| `IdMapIndex::remove` | `#remove` | Covered | Returns `Boolean` |
| `IdMapIndex::search` | `#search` | Covered | Returns `[scores, ids]` |
| `IdMapIndex::search_with_allowlist` | `#search_with_allowlist` | Covered | Returns `[scores, ids]` |
| `IdMapIndex::contains` | `#contains?` | Covered | Membership query |
| `IdMapIndex::prepare` | `#prepare` | Covered | Eager cache warm-up |
| `IdMapIndex::write` | `#write` | Covered | Serializes `.tvim` files |
| `IdMapIndex::len` | `#len` | Covered | Metadata |
| `IdMapIndex::is_empty` | `#empty?` | Covered | Metadata |
| `IdMapIndex::dim` / `dim_opt` | `#dim`, `#dim_opt` | Covered | Metadata |
| `IdMapIndex::bit_width` | `#bit_width` | Covered | Metadata |

## Public crate items not wrapped in Ruby yet

| Rust item | Ruby wrapper | Status | Notes |
| --- | --- | --- | --- |
| `codebook::codebook` | None | Not exposed | Low-level helper |
| `encode::encode` | None | Not exposed | Low-level helper |
| `io::write` / `io::load` | None | Not exposed | Internal serialization helpers |
| `io::write_id_map` / `io::load_id_map` | None | Not exposed | Internal serialization helpers |
| `pack::repack` / `pack::repack_3bit` | None | Not exposed | Internal packing helpers |
| `rotation::make_rotation_matrix` | None | Not exposed | Internal helper |
| `search::search` | None | Not exposed | Core kernel, already used indirectly |
| `search::blocks_skipped_by_mask` | None | Not exposed | Instrumentation |
| `search::reset_blocks_skipped_by_mask` | None | Not exposed | Instrumentation |
| Public modules `codebook`, `encode`, `io`, `pack`, `rotation`, `search` | None | Not exposed directly | Implemented via the higher-level Ruby classes |

## Notes

- The Ruby gem intentionally wraps the high-level APIs first, not every helper function.
- The CI workflow should validate the Ruby wrapper and native extension on Linux and macOS.
- The version watcher should open an issue when crates.io publishes a newer `turbovec` release than the locked Rust dependency.

