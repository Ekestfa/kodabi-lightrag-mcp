pub trait RagService {

}

#[derive(Debug)]
enum OperationsError {
    CanNotHandled,
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

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}