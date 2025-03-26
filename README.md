# text2num-py

Convert written out numbers (e.g. `"duizendnegenhonderddrieënzeventig"`) into digits (1973) using a fast Rust + Python binding.

Based on: https://github.com/allo-media/text2num-rs

Works for the following languages:
- Dutch ("nl")
- English ("en")
- French ("fr")
- Spanish ("es")
- German ("de")
- Italian ("it)

## Installation

pip install polytext2num


## Usage
```python
from polytext2num import text_to_number

print(text_to_number("duizendnegenhonderddrieënzeventig", "dutch"))    # → "1973"
print(text_to_number("one thousand nine hundred seventy-three", "en")) # → "1973"
print(text_to_number("quatre-vingt-dix", "french"))                    # → "90"
```