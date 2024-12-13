# pyo3-question

To replicate,

First,
```
make venv
```
to create a new local virtual python environment with spacy.

Then,
```
source venv/bin/activate
```
to activate the virtual python environment.

Lastly,
```
make run
```
to see the error
```
thread 'main' panicked at src/main.rs:15:48:
called `Result::unwrap()` on an `Err` value: PyErr { type: <class 'AttributeError'>, value: AttributeError("class must define a '_type_' attribute"), traceback: Some(<traceback object at 0x7f2553c45700>) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
