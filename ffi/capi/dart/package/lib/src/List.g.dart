// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// A list of strings
final class List implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  List._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XList_destroy'));

  /// Create a new list of strings
  factory List() {
    final result = _ICU4XList_create();
    return List._(result);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XList_create =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>('ICU4XList_create')
      .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Create a new list of strings with preallocated space to hold
  /// at least `capacity` elements
  factory List.withCapacity(int capacity) {
    final result = _ICU4XList_create_with_capacity(capacity);
    return List._(result);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XList_create_with_capacity =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>>('ICU4XList_create_with_capacity')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  /// Push a string to the list
  ///
  /// For C++ users, potentially invalid UTF8 will be handled via
  /// REPLACEMENT CHARACTERs
  void push(String val) {
    final temp = ffi2.Arena();
    final valView = val.utf8View;
    _ICU4XList_push(_underlying, valView.pointer(temp), valView.length);
    temp.releaseAll();
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XList_push =
    _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XList_push')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);

  /// The number of elements in this list
  int get length {
    final result = _ICU4XList_len(_underlying);
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XList_len =
    _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XList_len')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Whether this list is empty
  bool get isEmpty {
    final result = _ICU4XList_is_empty(_underlying);
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XList_is_empty =
    _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XList_is_empty')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
