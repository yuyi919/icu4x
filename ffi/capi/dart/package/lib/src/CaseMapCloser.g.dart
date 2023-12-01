// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `CaseMapCloser`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html) for more information.
final class CaseMapCloser implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  CaseMapCloser._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XCaseMapCloser_destroy'));

  /// Construct a new ICU4XCaseMapper instance
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory CaseMapCloser(DataProvider provider) {
    final result = _ICU4XCaseMapCloser_create(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return CaseMapCloser._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XCaseMapCloser_create =
    _capi<ffi.NativeFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XCaseMapCloser_create')
      .asFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Adds all simple case mappings and the full case folding for `c` to `builder`.
  /// Also adds special case closure mappings.
  ///
  /// See the [Rust documentation for `add_case_closure_to`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_case_closure_to) for more information.
  void addCaseClosureTo(Rune c, CodePointSetBuilder builder) {
    _ICU4XCaseMapCloser_add_case_closure_to(_underlying, c, builder._underlying);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XCaseMapCloser_add_case_closure_to =
    _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Pointer<ffi.Opaque>)>>('ICU4XCaseMapCloser_add_case_closure_to')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, Rune, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Finds all characters and strings which may casemap to `s` as their full case folding string
  /// and adds them to the set.
  ///
  /// Returns true if the string was found
  ///
  /// See the [Rust documentation for `add_string_case_closure_to`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_string_case_closure_to) for more information.
  bool addStringCaseClosureTo(String s, CodePointSetBuilder builder) {
    final temp = ffi2.Arena();
    final sView = s.utf8View;
    final result = _ICU4XCaseMapCloser_add_string_case_closure_to(_underlying, sView.pointer(temp), sView.length, builder._underlying);
    temp.releaseAll();
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XCaseMapCloser_add_string_case_closure_to =
    _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Opaque>)>>('ICU4XCaseMapCloser_add_string_case_closure_to')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
