# solana-bigtable-storage-models
This crate defines the object models stored in the bigtable via the Geyser plugin.
It can be used for both the  writing side and the reading side.
The models are defined under proto. After building, it will generate the Rust
object models.

To use the models, do the 'use' statement like the following,

```
use solana_bigtable_geyser_models::models::accounts;
```