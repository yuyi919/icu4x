// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An ICU4X Locale, capable of representing strings like `"en-US"`.
///
/// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
final class Locale implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Locale._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XLocale_destroy'));

  /// Construct an [`Locale`] from an locale identifier.
  ///
  /// This will run the complete locale parsing algorithm. If code size and
  /// performance are critical and the locale is of a known shape (such as
  /// `aa-BB`) use `create_und`, `set_language`, `set_script`, and `set_region`.
  ///
  /// See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.try_from_bytes) for more information.
  ///
  /// Throws [Error] on failure.
  factory Locale.fromString(String name) {
    final temp = ffi2.Arena();
    final nameView = name.utf8View;
    final result = _ICU4XLocale_create_from_string(nameView.pointer(temp), nameView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return Locale._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_create_from_string =
    _capi<ffi.NativeFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XLocale_create_from_string')
      .asFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);

  /// Construct a default undefined [`Locale`] "und".
  ///
  /// See the [Rust documentation for `UND`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#associatedconstant.UND) for more information.
  factory Locale.und() {
    final result = _ICU4XLocale_create_und();
    return Locale._(result);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_create_und =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>('ICU4XLocale_create_und')
      .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Clones the [`Locale`].
  ///
  /// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
  Locale clone() {
    final result = _ICU4XLocale_clone(_underlying);
    return Locale._(result);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_clone =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_clone')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Write a string representation of the `LanguageIdentifier` part of
  /// [`Locale`] to `write`.
  ///
  /// See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.id) for more information.
  ///
  /// Throws [Error] on failure.
  String get basename {
    final writeable = _Writeable();
    final result = _ICU4XLocale_basename(_underlying, writeable._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_basename =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_basename')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Write a string representation of the unicode extension to `write`
  ///
  /// See the [Rust documentation for `extensions`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.extensions) for more information.
  ///
  /// Throws [Error] on failure.
  String getUnicodeExtension(String bytes) {
    final temp = ffi2.Arena();
    final bytesView = bytes.utf8View;
    final writeable = _Writeable();
    final result = _ICU4XLocale_get_unicode_extension(_underlying, bytesView.pointer(temp), bytesView.length, writeable._underlying);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_get_unicode_extension =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_get_unicode_extension')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Write a string representation of [`Locale`] language to `write`
  ///
  /// See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.id) for more information.
  ///
  /// Throws [Error] on failure.
  String get language {
    final writeable = _Writeable();
    final result = _ICU4XLocale_language(_underlying, writeable._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_language =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_language')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Set the language part of the [`Locale`].
  ///
  /// See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.try_from_bytes) for more information.
  ///
  /// Throws [Error] on failure.
  set language(String bytes) {
    final temp = ffi2.Arena();
    final bytesView = bytes.utf8View;
    final result = _ICU4XLocale_set_language(_underlying, bytesView.pointer(temp), bytesView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_set_language =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XLocale_set_language')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);

  /// Write a string representation of [`Locale`] region to `write`
  ///
  /// See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.id) for more information.
  ///
  /// Throws [Error] on failure.
  String get region {
    final writeable = _Writeable();
    final result = _ICU4XLocale_region(_underlying, writeable._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_region =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_region')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Set the region part of the [`Locale`].
  ///
  /// See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.try_from_bytes) for more information.
  ///
  /// Throws [Error] on failure.
  set region(String bytes) {
    final temp = ffi2.Arena();
    final bytesView = bytes.utf8View;
    final result = _ICU4XLocale_set_region(_underlying, bytesView.pointer(temp), bytesView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_set_region =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XLocale_set_region')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);

  /// Write a string representation of [`Locale`] script to `write`
  ///
  /// See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.id) for more information.
  ///
  /// Throws [Error] on failure.
  String get script {
    final writeable = _Writeable();
    final result = _ICU4XLocale_script(_underlying, writeable._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_script =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_script')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Set the script part of the [`Locale`]. Pass an empty string to remove the script.
  ///
  /// See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.try_from_bytes) for more information.
  ///
  /// Throws [Error] on failure.
  set script(String bytes) {
    final temp = ffi2.Arena();
    final bytesView = bytes.utf8View;
    final result = _ICU4XLocale_set_script(_underlying, bytesView.pointer(temp), bytesView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_set_script =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XLocale_set_script')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);

  /// Best effort locale canonicalizer that doesn't need any data
  ///
  /// Use ICU4XLocaleCanonicalizer for better control and functionality
  ///
  /// See the [Rust documentation for `canonicalize`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.canonicalize) for more information.
  ///
  /// Throws [Error] on failure.
  static String canonicalize(String bytes) {
    final temp = ffi2.Arena();
    final bytesView = bytes.utf8View;
    final writeable = _Writeable();
    final result = _ICU4XLocale_canonicalize(bytesView.pointer(temp), bytesView.length, writeable._underlying);
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_canonicalize =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_canonicalize')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Uint8>, int, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Write a string representation of [`Locale`] to `write`
  ///
  /// See the [Rust documentation for `write_to`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.write_to) for more information.
  ///
  /// Throws [Error] on failure.
  @override
  String toString() {
    final writeable = _Writeable();
    final result = _ICU4XLocale_to_string(_underlying, writeable._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return writeable.finalize();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_to_string =
    _capi<ffi.NativeFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_to_string')
      .asFunction<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `normalizing_eq`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.normalizing_eq) for more information.
  bool normalizingEq(String other) {
    final temp = ffi2.Arena();
    final otherView = other.utf8View;
    final result = _ICU4XLocale_normalizing_eq(_underlying, otherView.pointer(temp), otherView.length);
    temp.releaseAll();
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_normalizing_eq =
    _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XLocale_normalizing_eq')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);

  /// See the [Rust documentation for `strict_cmp`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.strict_cmp) for more information.
  Ordering strictCmp(String other) {
    final temp = ffi2.Arena();
    final otherView = other.utf8View;
    final result = _ICU4XLocale_strict_cmp(_underlying, otherView.pointer(temp), otherView.length);
    temp.releaseAll();
    return Ordering.values.firstWhere((v) => v._underlying == result);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XLocale_strict_cmp =
    _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XLocale_strict_cmp')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);
}
