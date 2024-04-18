use pyo3::prelude::*;
//use pyo3::types::IntoPyDict;

fn infer(prompt: String) -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let _sys = py.import("sys")?;
    let _os = py.import("os")?;
    let transformers = py.import("transformers")?;

    let model = transformers.getattr("BertForMaskedLM")?.call_method1("from_pretrained", ("bert-base-uncased",))?;
    let tokenizer = transformers.getattr("BertTokenizer")?.call_method1("from_pretrained", ("bert-base-uncased",))?;

    let inputs = tokenizer.call_method1("encode_plus", (prompt,))?;
    let outputs = model.call_method1("__call__", (inputs.get_item(0)?,))?;

    let logits = outputs.get_item(0)?;
    let predicted_index = logits.call_method1("argmax", (py.None(),))?;
    let predicted_token = tokenizer.call_method1("convert_ids_to_tokens", (predicted_index,))?;

    println!("Predicted token: {:?}", predicted_token);

    Ok(())
}

fn main() {
    let prompt = "Hello, world!".to_string();
    match infer(prompt) {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {:?}", e),
    }
}