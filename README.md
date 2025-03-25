# text2num-py

Convert Dutch number words (e.g. `"duizendnegenhonderddrieënzeventig"`) into digits using Rust + Python.

## Installation

```bash
pip install git+https://github.com/your-username/text2num_py.git
```

## Usage
```python
import text2num_py

print(text2num_py.dutch_to_number("duizendnegenhonderddrieënzeventig"))
# Should output: "1973"
```