# API Documentation

## Base URL

```
http://localhost:8080
```

## Endpoints

### Health Check

```http
GET /api/health
```

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

### Start Scan

```http
POST /api/scan
```

**Response:**
```json
{
  "success": true,
  "device_count": 5,
  "message": "Discovered 5 devices"
}
```

### Get Devices

```http
GET /api/devices
```

**Response:**
```json
{
  "devices": [
    {
      "id": "uuid",
      "mac_address": "AA:BB:CC:DD:EE:FF",
      "vendor": "Espressif",
      "ssid": "MyNetwork",
      "bssid": "AA:BB:CC:DD:EE:FF",
      "rssi": -45,
      "channel": 6,
      "security": "WPA2",
      "device_type": "ESP32",
      "first_seen": "2024-01-01T00:00:00Z",
      "last_seen": "2024-01-01T00:00:00Z",
      "is_active": true,
      "estimated_uptime_seconds": 3600
    }
  ]
}
```

### Get Device

```http
GET /api/devices/:id
```

**Response:**
```json
{
  "id": "uuid",
  "mac_address": "AA:BB:CC:DD:EE:FF",
  "vendor": "Espressif",
  "ssid": "MyNetwork",
  "rssi": -45,
  "channel": 6,
  "security": "WPA2",
  "device_type": "ESP32"
}
```

### Evaluate Risk

```http
POST /api/risk/evaluate
Content-Type: application/json

{
  "device_id": "uuid"
}
```

**Response:**
```json
{
  "device_id": "uuid",
  "risk_score": {
    "total_score": 35,
    "level": "Medium",
    "applied_rules": [
      {
        "id": "GW-RF-001",
        "name": "Hardware compatível com CSI",
        "description": "Detectado hardware compatível com CSI",
        "score": 10,
        "conditions": {}
      }
    ],
    "recommendations": [
      "Monitorar tráfego de rede para padrões suspeitos"
    ]
  }
}
```

### Get Exposure

```http
GET /api/exposure
```

**Response:**
```json
{
  "exposure_score": {
    "score": 45,
    "metrics": {
      "wifi_coverage": 0.8,
      "ap_count": 3,
      "device_count": 10,
      "rf_density": 0.5,
      "spatial_distribution": 0.6
    },
    "factors": [
      "Alta cobertura Wi-Fi detectada",
      "Múltiplos APs detectados: 3"
    ]
  }
}
```

### Generate Report

```http
POST /api/reports
Content-Type: application/json

{
  "format": "html"
}
```

**Response:**
```json
{
  "report_id": "uuid",
  "content": "<html>...</html>"
}
```

### Get Threat Intelligence

```http
GET /api/threat-intel
```

**Response:**
```json
{
  "signatures": [
    {
      "id": "THREAT-001",
      "vendor": "Espressif",
      "fingerprint_pattern": "ESP32-S3.*continuous.*mqtt",
      "risk_score": 30,
      "tags": ["csi-compatible", "iot"],
      "version": "1.0.0",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "version": "1.0.0",
  "last_updated": "2024-01-01T00:00:00Z"
}
```

## Error Responses

All endpoints may return error responses:

```json
{
  "error": "Error message"
}
```

HTTP Status Codes:
- `200` - Success
- `400` - Bad Request
- `404` - Not Found
- `500` - Internal Server Error
