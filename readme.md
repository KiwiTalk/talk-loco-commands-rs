# talk-loco-commands
Official client/server compatible Loco commands implementation. Check doc for additional description.

## Contributing
Check `src/request`, `src/response` directory for already implemented command datas.
For data structs used in many places check `src/structs`.

Example bson command data implementation.
```rust
use crate::BsonData;
use serde::{Serialize, Deserialize};

/* 
 * Command method will be SAMPLEDATA in this case.
 * Note the Method name will be sliced to 11 bytes.
 */
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SampleData {
    
    pub status: i16,
    pub message: String

}
```

## License
```
MIT License

Copyright (c) 2020 storycraft

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```