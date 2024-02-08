use io::ScrowMetadata;
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;

fn main(){
   WasmBuilder::with_meta(ScrowMetadata::repr()).exclude_features(vec!["binary-vendor"]).build();
}