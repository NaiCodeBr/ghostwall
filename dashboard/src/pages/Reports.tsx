import { useState } from "react";
import { Download, FileText } from "lucide-react";

function Reports() {
  const [format, setFormat] = useState("json");

  const generateReport = async () => {
    try {
      const response = await fetch("http://localhost:8080/api/reports", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ format }),
      });
      const data = await response.json();
      
      // Download the report
      const blob = new Blob([data.content], { type: "text/plain" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `ghostwall-report.${format}`;
      a.click();
    } catch (error) {
      console.error("Failed to generate report:", error);
    }
  };

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-8">Reports</h1>
      
      <div className="bg-gray-800 rounded-lg p-6 mb-6">
        <h2 className="text-xl font-semibold mb-4">Generate Report</h2>
        
        <div className="mb-4">
          <label className="block text-sm font-medium text-gray-300 mb-2">
            Format
          </label>
          <select
            value={format}
            onChange={(e) => setFormat(e.target.value)}
            className="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:border-green-500"
          >
            <option value="json">JSON</option>
            <option value="html">HTML</option>
            <option value="pdf">PDF</option>
          </select>
        </div>
        
        <button
          onClick={generateReport}
          className="flex items-center px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg transition-colors"
        >
          <Download className="w-5 h-5 mr-2" />
          Generate Report
        </button>
      </div>

      <div className="bg-gray-800 rounded-lg p-6">
        <h2 className="text-xl font-semibold mb-4">Recent Reports</h2>
        <div className="flex items-center text-gray-400">
          <FileText className="w-5 h-5 mr-2" />
          <p>No recent reports available.</p>
        </div>
      </div>
    </div>
  );
}

export default Reports;
