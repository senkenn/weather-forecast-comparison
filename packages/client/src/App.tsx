import { ResponsiveLine } from "@nivo/line";
import { useState } from "react";

const data = [
  {
    id: "Dataset 1",
    data: [
      { x: "Mon", y: 120 },
      { x: "Tue", y: 132 },
      { x: "Wed", y: 101 },
      { x: "Thu", y: 134 },
      { x: "Fri", y: 90 },
      { x: "Sat", y: 230 },
      { x: "Sun", y: 210 },
    ],
  },
  {
    id: "Dataset 2",
    data: [
      { x: "Mon", y: 220 },
      { x: "Tue", y: 182 },
      { x: "Wed", y: 191 },
      { x: "Thu", y: 234 },
      { x: "Fri", y: 290 },
      { x: "Sat", y: 330 },
      { x: "Sun", y: 310 },
    ],
  },
];

function NivoLineChart() {
  const [visibleDatasets, setVisibleDatasets] = useState({
    "Dataset 1": true,
    "Dataset 2": true,
  });

  const handleLegendClick = (datasetId) => {
    setVisibleDatasets((prev) => ({
      ...prev,
      [datasetId]: !prev[datasetId],
    }));
  };

  const filteredData = data.filter((dataset) => visibleDatasets[dataset.id]);

  return (
    <div style={{ height: 400 }}>
      <h3>Nivo Line Chart with Legend Toggle</h3>
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          marginBottom: "1em",
        }}
      >
        {Object.keys(visibleDatasets).map((datasetId) => (
          <button
            key={datasetId}
            onClick={() => handleLegendClick(datasetId)}
            style={{
              backgroundColor: visibleDatasets[datasetId] ? "#ccc" : "#fff",
              border: "1px solid #ddd",
              padding: "5px 10px",
              margin: "0 5px",
              cursor: "pointer",
            }}
          >
            {datasetId}
          </button>
        ))}
      </div>
      <ResponsiveLine
        data={filteredData}
        margin={{ top: 50, right: 110, bottom: 50, left: 60 }}
        xScale={{ type: "point" }}
        yScale={{
          type: "linear",
          min: "auto",
          max: "auto",
          stacked: true,
          reverse: false,
        }}
        axisTop={null}
        axisRight={null}
        axisBottom={{
          orient: "bottom",
          tickSize: 5,
          tickPadding: 5,
          tickRotation: 0,
          legend: "Day",
          legendOffset: 36,
          legendPosition: "middle",
        }}
        axisLeft={{
          orient: "left",
          tickSize: 5,
          tickPadding: 5,
          tickRotation: 0,
          legend: "Value",
          legendOffset: -40,
          legendPosition: "middle",
        }}
        colors={{ scheme: "nivo" }}
        pointSize={10}
        pointColor={{ theme: "background" }}
        pointBorderWidth={2}
        pointBorderColor={{ from: "serieColor" }}
        pointLabelYOffset={-12}
        useMesh={true}
      />
    </div>
  );
}

export default NivoLineChart;
