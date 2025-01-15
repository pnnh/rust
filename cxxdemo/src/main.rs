#[cxx::bridge]
mod ffi {
    struct ConcatRequest {
        fst: String,
        snd: String,
    }

    unsafe extern "C++" {
        include!("cxxdemo/include/blobstore.h");
        include!("cxxdemo/include/concat.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn concat(r: ConcatRequest) -> String;
    }
}

fn main() {
    println!("Hello, world!");
    let client = ffi::new_blobstore_client();
    let concatenated = ffi::concat(ffi::ConcatRequest {
        fst: "fearless".to_owned(),
        snd: "concurrency".to_owned(),
    });
    println!("concatenated: {:?}", concatenated);
}
