import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class Device {
  final String id;
  final String macAddress;
  final String? vendor;
  final String? ssid;
  final int rssi;
  final int channel;
  final String deviceType;

  Device({
    required this.id,
    required this.macAddress,
    this.vendor,
    this.ssid,
    required this.rssi,
    required this.channel,
    required this.deviceType,
  });

  factory Device.fromJson(Map<String, dynamic> json) {
    return Device(
      id: json['id'],
      macAddress: json['mac_address'],
      vendor: json['vendor'],
      ssid: json['ssid'],
      rssi: json['rssi'],
      channel: json['channel'],
      deviceType: json['device_type'],
    );
  }
}

class DeviceProvider with ChangeNotifier {
  List<Device> _devices = [];
  bool _isLoading = false;
  String _error = '';

  List<Device> get devices => _devices;
  bool get isLoading => _isLoading;
  String get error => _error;

  Future<void> fetchDevices() async {
    _isLoading = true;
    _error = '';
    notifyListeners();

    try {
      final response = await http.get(
        Uri.parse('http://localhost:8080/api/devices'),
      );

      if (response.statusCode == 200) {
        final data = json.decode(response.body);
        _devices = (data['devices'] as List)
            .map((device) => Device.fromJson(device))
            .toList();
      } else {
        _error = 'Failed to load devices';
      }
    } catch (e) {
      _error = 'Network error: $e';
    } finally {
      _isLoading = false;
      notifyListeners();
    }
  }

  Future<void> startScan() async {
    _isLoading = true;
    notifyListeners();

    try {
      final response = await http.post(
        Uri.parse('http://localhost:8080/api/scan'),
      );

      if (response.statusCode == 200) {
        await fetchDevices();
      } else {
        _error = 'Scan failed';
      }
    } catch (e) {
      _error = 'Network error: $e';
    } finally {
      _isLoading = false;
      notifyListeners();
    }
  }
}
