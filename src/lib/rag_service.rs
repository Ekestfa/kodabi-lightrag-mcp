pub trait RagService {

}

#[derive(debug, clone)]
pub struct RagServices {
    rag_array: Vec<Rag>
}

#[derive(debug, clone)]
pub struct Rag {
    rag_name: string,
    rag_ip: string,
    rag_port: string
}