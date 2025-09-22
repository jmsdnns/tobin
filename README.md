![Tobin](logo.png)

🦀 _A simple, minimal key-value store_


## Why

I have been reading [Designing Data-Intensive Applications](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781491903063/) and feeling very inspired to build some of what I have read about. This project is an attempt to build a simple kv-store based on LSM trees.

This project exists simply for the joy of learning.


## The Plan

I haven't built a kv store before, so I cannot claim to know exactly what I'm doing. I worked with Claude to develop the plan below and I'm going to try to follow it.

The idea is to build an LSM-tree based kv-store. That will take several steps, grouped into phases.

**Phase 1: Basic Foundation**
1. Define core interfaces: KV operations (get, put, delete), key/value serialization
2. Implement in-memory MemTable using skip list or balanced tree
3. Add basic WAL for crash recovery (append-only log file)
4. Create simple SSTable writer with sorted key-value blocks
5. Implement SSTable reader with sequential scan capability

**Phase 2: Read Path**
1. Add SSTable index blocks for efficient key lookups
2. Implement basic bloom filters per SSTable
3. Build read path: MemTable → SSTables in reverse chronological order
4. Add tombstone handling for deletions
5. Implement range scan across MemTable and SSTables

**Phase 3: Write Path & Flushing**
1. Add MemTable size-based flush triggers
2. Implement immutable MemTable queue during flush
3. Create SSTable metadata (key ranges, file info)
4. Add manifest file to track all SSTables
5. Implement atomic SSTable installation after flush

**Phase 4: Basic Compaction**
1. Start with size-tiered compaction (simpler than leveled)
2. Implement merge logic for overlapping SSTables
3. Add background compaction thread
4. Handle compaction triggers based on file count/size
5. Update manifest atomically after compaction

**Phase 5: Optimization**
1. Add block-based compression to SSTables
2. Implement block cache for hot data
3. Optimize bloom filter false positive rates
4. Add compaction parallelization
5. Tune MemTable and SSTable sizing

**Phase 6: Reliability**
1. Improve crash recovery robustness
2. Add data integrity checks (checksums)
3. Handle disk space exhaustion gracefully
4. Implement proper resource cleanup
5. Add comprehensive error handling

**Phase 7: Advanced Features**
1. Switch to leveled compaction for better read performance
2. Add range deletions
3. Implement snapshots/backups
4. Add metrics and monitoring
5. Performance profiling and optimization

