# polytext2num

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
from polytext2num import txt2num

print(txt2num("duizendnegenhonderddrieënzeventig", "dutch"))    
# "1973"

print(txt2num("one thousand nine hundred seventy-three", "en")) 
# "1973"

print(txt2num("quatre-vingt-dix", "french")) 
# "90"

print(txt2num("I have two apples and one banana, twenty kegs.", "en", threshold = 10)) 
# "I have two apples and one banana, 20 kegs."
```