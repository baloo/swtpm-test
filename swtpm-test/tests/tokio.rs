use std::future::ready;
use tss_esapi::Context;

#[swtpm_test::test]
#[tokio::main]
async fn test_async(context: &mut Context) {
    // Something something async
    ready(()).await;
    // Something something with context
    let _ = context;
}
