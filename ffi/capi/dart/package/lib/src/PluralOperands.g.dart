// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// FFI version of `PluralOperands`.
///
/// See the [Rust documentation for `PluralOperands`](https://docs.rs/icu/latest/icu/plurals/struct.PluralOperands.html) for more information.
final class PluralOperands implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  PluralOperands._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XPluralOperands_destroy'));

  /// Construct for a given string representing a number
  ///
  /// See the [Rust documentation for `from_str`](https://docs.rs/icu/latest/icu/plurals/struct.PluralOperands.html#method.from_str) for more information.
  ///
  /// Throws [Error] on failure.
  factory PluralOperands.fromString(String s) {
    final temp = ffi2.Arena();
    final sView = s.utf8View;
    final result = _ICU4XPluralOperands_create_from_string(sView.pointer(temp), sView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return PluralOperands._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XPluralOperands_create_from_string =
    _capi<ffi.NativeFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XPluralOperands_create_from_string')
      .asFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);
}
