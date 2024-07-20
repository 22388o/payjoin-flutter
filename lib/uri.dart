import 'package:payjoin_flutter/src/config.dart';
import 'package:payjoin_flutter/src/exceptions.dart';

import 'src/generated/api/io.dart' as io;
import 'src/generated/api/uri.dart';
import 'src/generated/utils/error.dart' as error;

class PjUriBuilder extends FfiPjUriBuilder {
  PjUriBuilder({required super.internal});

  ///Create a new PjUriBuilder with required parameters.
  /// Parameters
  /// address: Represents a bitcoin address.
  /// ohttpKeys: Optional OHTTP keys for v2.
  /// expiry: Optional non-default duration_since epoch expiry for the `payjoin` session.
  static Future<PjUriBuilder> create(
      {required String address,
      required FfiUrl pj,
      FfiOhttpKeys? ohttpKeys,
      BigInt? expiry}) async {
    try {
      await PConfig.initializeApp();
      final res = await FfiPjUriBuilder.create(
          address: address, pj: pj, ohttpKeys: ohttpKeys, expiry: expiry);
      return PjUriBuilder(internal: res.internal);
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }
}

class Uri extends FfiUri {
  Uri._({required super.field0});

  static Future<Uri> fromString(String uri) async {
    try {
      await PConfig.initializeApp();
      final res = await FfiUri.fromStr(uri: uri);
      return Uri._(field0: res.field0);
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }

  ///Gets the amount in btc.
  @override
  double? amount({hint}) {
    try {
      return super.amount();
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }

  @override
  String address({hint}) {
    try {
      return super.address();
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }

  @override
  PjUri checkPjSupported({hint}) {
    try {
      return PjUri._(field0: super.checkPjSupported().field0);
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }

  @override
  String toString() {
    return super.asString();
  }
}

class PjUri extends FfiPjUri {
  PjUri._({required super.field0});

  ///Gets the amount in btc.
  @override
  double? amount({hint}) {
    try {
      return super.amount();
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }

  @override
  String address({hint}) {
    try {
      return super.address();
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }

  @override
  String toString() {
    return super.asString();
  }
}

class Url extends FfiUrl {
  Url._({required super.field0});
  static Future<Url> fromString(String uri) async {
    try {
      await PConfig.initializeApp();
      final res = await FfiUrl.fromStr(url: uri);
      return Url._(field0: res.field0);
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }

  @override
  String? query({hint}) {
    try {
      return super.query();
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }

  @override
  String toString() {
    return super.asString();
  }
}

///The key configuration of a server. This can be used by both client and server.
///An important invariant of this structure is that it does not include any combination of KEM, KDF, and AEAD that is not supported.
class OhttpKeys extends FfiOhttpKeys {
  OhttpKeys._({required super.field0});

  ///Construct a configuration from the encoded server configuration.
  static Future<OhttpKeys> decode({required List<int> bytes}) async {
    try {
      final res = await FfiOhttpKeys.decode(bytes: bytes);
      return OhttpKeys._(field0: res.field0);
    } on error.PayjoinError catch (e) {
      throw mapPayjoinError(e);
    }
  }
}

/// Fetch the ohttp keys from the specified payjoin directory via proxy.
///
/// * `ohttpRelay`: The http CONNNECT method proxy to request the ohttp keys from a payjoin
/// directory.  Proxying requests for ohttp keys ensures a client IP address is never revealed to
/// the payjoin directory.
///
/// * `payjoinDirectory`: The payjoin directory from which to fetch the ohttp keys.  This
/// directory stores and forwards payjoin client payloads.
///
/// * `certDer` (optional): The DER-encoded certificate to use for local HTTPS connections.  This
/// parameter is only available when the "danger-local-https" feature is enabled.
Future<OhttpKeys> fetchOhttpKeys({
  required Url ohttpRelay,
  required Url payjoinDirectory,
}) async {
  try {
    final res = await io.fetchOhttpKeys(
      ohttpRelay: ohttpRelay,
      payjoinDirectory: payjoinDirectory,
    );
    return OhttpKeys._(field0: res.field0);
  } on error.PayjoinError catch (e) {
    throw mapPayjoinError(e);
  }
}
