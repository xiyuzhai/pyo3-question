use pyo3::{prelude::*, types::PyDict};

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let code = std::ffi::CString::new(
            r#"
import spacy
"#,
        )
        .unwrap();

        let code_cstr = code.as_c_str();

        let locals = PyDict::new(py);
        py.run(code_cstr, None, Some(&locals)).unwrap();

        Ok(())
    })
}
