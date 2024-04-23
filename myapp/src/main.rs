use lambda_runtime::{handler_fn, Context, Error};
use pyo3::prelude::*;
use serde_json::Value;

async fn infer(event: Value, _: Context) -> Result<(), Error> {
    let prompt = event["prompt"].as_str().unwrap().to_string();
    let gil = Python::acquire_gil();
    let py = gil.python();
    let _sys = py.import("sys")?;
    let _os = py.import("os")?;
    let transformers = py.import("transformers")?;

    let model = transformers.getattr("BertForMaskedLM")?.call_method1("from_pretrained", ("bert-base-uncased",))?;
    let tokenizer = transformers.getattr("BertTokenizer")?.call_method1("from_pretrained", ("bert-base-uncased",))?;

    let inputs = tokenizer.call_method1("encode_plus", (prompt,))?;
    println!("Inputs: {:?}", inputs);
    let torch = py.import("torch")?;

    let input_ids = inputs.get_item("input_ids")?;
    let input_ids = torch.call_method1("tensor", (input_ids,))?;
    
    let _token_type_ids = inputs.get_item("token_type_ids")?;
    let _token_type_ids = torch.call_method1("tensor", (_token_type_ids,))?;
    
    let attention_mask = inputs.get_item("attention_mask")?;
    let attention_mask = torch.call_method1("tensor", (attention_mask,))?;
    let input_ids = input_ids.call_method1("unsqueeze", (0,))?;
    let _token_type_ids = _token_type_ids.call_method1("unsqueeze", (0,))?;
    let attention_mask = attention_mask.call_method1("unsqueeze", (0,))?;
    
    let outputs = model.call_method1("__call__", (input_ids, None::<Py<PyAny>>, None::<Py<PyAny>>, attention_mask, None::<Py<PyAny>>, None::<Py<PyAny>>, None::<Py<PyAny>>, false))?;    
    let _logits = outputs.get_item(0)?;
    let _logits = outputs.get_item(0)?;
    let predicted_index = _logits.call_method1("argmax", (-1,))?;
    let predicted_index = predicted_index.call_method1("tolist", ())?;
    let predicted_index = predicted_index.get_item(0)?;
    let predicted_token = tokenizer.call_method1("convert_ids_to_tokens", (predicted_index,))?;

    println!("Predicted token: {:?}", predicted_token);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    pyo3::prepare_freethreaded_python();
    let func = handler_fn(infer);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_infer() {
        pyo3::prepare_freethreaded_python();
        let event = json!({
            "prompt": "Hello, world!"
        });
        let context = Context::default();
        let result = infer(event, context).await;
        assert!(result.is_ok());
    }
}