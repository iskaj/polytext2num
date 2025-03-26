# text2num-py

Convert Dutch written out numbers (e.g. `"duizendnegenhonderddrieënzeventig"`) into digits using Rust + Python.

## Installation

```bash
pip install dutchtext2num
```

## Usage
```python
import dutchtext2num

print(dutchtext2num.dutch_to_number("duizendnegenhonderddrieënzeventig"))
# Should output: "1973"
```