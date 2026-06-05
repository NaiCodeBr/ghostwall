import { useState, useEffect } from "react";

function ExposureMap() {
  const [exposureScore, setExposureScore] = useState(0);

  useEffect(() => {
    fetchExposure();
  }, []);

  const fetchExposure = async () => {
    try {
      const response = await fetch("http://localhost:8080/api/exposure");
      const data = await response.json();
      setExposureScore(data.exposure_score.score);
    } catch (error) {
      console.error("Failed to fetch exposure:", error);
    }
  };

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-8">Exposure Map</h1>
      
      <div className="bg-gray-800 rounded-lg p-6 mb-6">
        <h2 className="text-xl font-semibold mb-4">RF Exposure Score</h2>
        <div className="text-5xl font-bold text-green-400">{exposureScore}/100</div>
      </div>

      <div className="bg-gray-800 rounded-lg p-6">
        <h2 className="text-xl font-semibold mb-4">Spatial Distribution</h2>
        <p className="text-gray-400">Exposure map visualization would be displayed here.</p>
      </div>
    </div>
  );
}

export default ExposureMap;
