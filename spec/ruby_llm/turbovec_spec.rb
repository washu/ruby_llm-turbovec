# frozen_string_literal: true

require "tempfile"

# rubocop:disable Metrics/BlockLength

RSpec.describe RubyLLM::Turbovec::TurboQuantIndex do
  let(:vector) { [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0] }

  it "has a version number" do
    expect(RubyLLM::Turbovec::VERSION).not_to be nil
  end

  it "creates an eager TurboQuantIndex and searches it" do
    index = described_class.new(8, 4)

    expect(index.dim).to eq(8)
    expect(index.bit_width).to eq(4)
    expect(index.empty?).to be(true)

    index.add(vector)

    results = index.search(vector, 1)

    expect(results.nq).to eq(1)
    expect(results.k).to eq(1)
    expect(results.indices_for_query(0)).to eq([0])
    expect(results.scores_for_query(0).first).to be_a(Float)
  end

  it "creates a lazy TurboQuantIndex, writes it, and reloads it" do
    index = described_class.new_lazy(4)
    expect(index.dim_opt).to be_nil

    index.add_with_dim(vector, 8)
    expect(index.dim).to eq(8)
    expect(index.len).to eq(1)

    Tempfile.create(["ruby_llm_turbovec", ".tv"]) do |file|
      path = file.path
      index.write(path)

      loaded = described_class.load(path)
      expect(loaded.len).to eq(1)
      expect(loaded.search(vector, 1).indices_for_query(0)).to eq([0])
    end
  end

  it "rejects invalid mask lengths before calling into rust" do
    index = described_class.new(8, 4)
    index.add(vector)

    expect do
      index.search_with_mask(vector, 1, [true, false])
    end.to raise_error(ArgumentError, /mask length/)
  end

  it "raises when lazy add is called without a dimension" do
    index = described_class.new_lazy(4)

    expect do
      index.add(vector)
    end.to raise_error(ArgumentError, /index dimension is not set/)
  end

  it "swaps out vectors in constant time" do
    index = described_class.new(8, 4)
    index.add(vector + vector)

    expect(index.swap_remove(0)).to eq(1)
    expect(index.len).to eq(1)
  end
end

RSpec.describe RubyLLM::Turbovec::IdMapIndex do
  let(:vectors) { [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0] * 2 }
  let(:ids) { [1001, 1002] }

  it "stores stable ids and searches them" do
    index = described_class.new(8, 4)

    index.add_with_ids(vectors, ids)

    scores, result_ids = index.search(vectors.first(8), 2)

    expect(scores.length).to eq(2)
    expect(result_ids).to match_array(ids)
    expect(index.contains?(1001)).to be(true)
  end

  it "supports lazy construction, allowlists, and persistence" do
    index = described_class.new_lazy(4)
    index.add_with_ids_2d(vectors, 8, ids)

    scores, result_ids = index.search_with_allowlist(vectors.first(8), 2, [1002])

    expect(scores.length).to eq(1)
    expect(result_ids).to eq([1002])

    Tempfile.create(["ruby_llm_turbovec", ".tvim"]) do |file|
      path = file.path
      index.write(path)

      loaded = described_class.load(path)
      expect(loaded.len).to eq(2)
      expect(loaded.contains?(1001)).to be(true)
      expect(loaded.search(vectors.first(8), 2).last).to match_array(ids)
    end
  end

  it "removes ids and keeps the remaining mapping valid" do
    index = described_class.new(8, 4)
    index.add_with_ids(vectors, ids)

    expect(index.remove(1001)).to be(true)
    expect(index.contains?(1001)).to be(false)
    expect(index.len).to eq(1)
    expect(index.search(vectors.first(8), 1).last).to eq([1002])
  end
end
# rubocop:enable Metrics/BlockLength
