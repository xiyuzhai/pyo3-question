use pyo3::{prelude::*, types::PyDict};

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let code = std::ffi::CString::new(
            r#"
from dataclasses import dataclass
import spacy

@dataclass
class Person:
    name: str
    age: int

person = Person("Alice", 30)
print(person)
"#,
        )
        .unwrap();

        let code_cstr = code.as_c_str();

        let locals = PyDict::new(py);
        py.run(code_cstr, None, Some(&locals)).unwrap();

        Ok(())
    })
}
