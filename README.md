# text2num-py

Convert Dutch written out numbers (e.g. `"duizendnegenhonderddrieënzeventig"`) into digits using Rust + Python.

## Installation

```bash
pip install dutchtext2num
```

## Usage
```python
from polytext2num import text_to_number

print(text_to_number("duizendnegenhonderddrieënzeventig", "dutch"))    # → "1973"
print(text_to_number("one thousand nine hundred seventy-three", "en")) # → "1973"
print(text_to_number("quatre-vingt-dix", "french"))                    # → "90"
```