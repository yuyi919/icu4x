
import {ICU4XLocale, ICU4XDataProvider, ICU4XLineSegmenter, ICU4XFixedDecimalFormatter, ICU4XFixedDecimal } from './lib/index.mjs';

const locale = ICU4XLocale.create_from_string("zh");
const provider = ICU4XDataProvider.create_compiled();

const format = ICU4XFixedDecimalFormatter.create_with_grouping_strategy(provider, locale, "Auto");

const decimal = ICU4XFixedDecimal.create_from_i32(1000007);
decimal.multiply_pow10(-2);

const result = format.format(decimal);
console.log("Output is", result);