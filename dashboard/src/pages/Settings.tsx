function Settings() {
  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-8">Settings</h1>
      
      <div className="space-y-6">
        <div className="bg-gray-800 rounded-lg p-6">
          <h2 className="text-xl font-semibold mb-4">General Settings</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Scan Interval (seconds)
              </label>
              <input
                type="number"
                defaultValue="60"
                className="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:border-green-500"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                API Endpoint
              </label>
              <input
                type="text"
                defaultValue="http://localhost:8080"
                className="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:border-green-500"
              />
            </div>
          </div>
        </div>

        <div className="bg-gray-800 rounded-lg p-6">
          <h2 className="text-xl font-semibold mb-4">Privacy Settings</h2>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium">Local Processing Only</h3>
                <p className="text-sm text-gray-400">All data processed locally</p>
              </div>
              <input
                type="checkbox"
                defaultChecked
                className="w-5 h-5 rounded"
              />
            </div>
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium">Encrypt Database</h3>
                <p className="text-sm text-gray-400">Encrypt local database with AES-256-GCM</p>
              </div>
              <input
                type="checkbox"
                defaultChecked
                className="w-5 h-5 rounded"
              />
            </div>
          </div>
        </div>

        <div className="bg-gray-800 rounded-lg p-6">
          <h2 className="text-xl font-semibold mb-4">About</h2>
          <div className="space-y-2 text-gray-300">
            <p><strong>Version:</strong> 0.1.0</p>
            <p><strong>License:</strong> MIT OR Apache-2.0</p>
            <p><strong>Repository:</strong> https://github.com/NaiCodeBr/ghostwall</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Settings;
