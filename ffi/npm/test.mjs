
import { ICU4XDataProvider, ICU4XLineSegmenter, ICU4XGraphemeClusterSegmenter } from './lib/index';
const text = "abc"
const provider = ICU4XDataProvider.create_compiled();
const segmenter = ICU4XGraphemeClusterSegmenter.create(provider)

const segments = [];

let utf8Index = 0;
let utf16Index = 0;
const iter8 = segmenter.segment_utf8(text);
while (true) {
    const next = iter8.next();

    if (next === -1) {
        segments.push(text.slice(utf16Index));
        break;
    } else {
        const oldUtf16Index = utf16Index;
        while (utf8Index < next) {
            const codePoint = text.codePointAt(utf16Index);
            const utf8Len = (codePoint <= 0x7F) ? 1
                : (codePoint <= 0x7FF) ? 2
                    : (codePoint <= 0xFFFF) ? 3
                        : 4;
            const utf16Len = (codePoint <= 0xFFFF) ? 1
                : 2;
            utf8Index += utf8Len;
            utf16Index += utf16Len;
        }
        segments.push(text.slice(oldUtf16Index, utf16Index));
    }
}
console.log(segments)