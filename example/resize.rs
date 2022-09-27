use dingding_ctrl::texture_batchresize;
use futures::executor::block_on;

fn main() {
    let _ = block_on(texture_batchresize::resize_by_yml(
        "example/static/config_android.yaml",
    ));
}
