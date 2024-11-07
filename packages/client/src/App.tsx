import * as echarts from "echarts";
import { useEffect, useRef, useState } from "react";

function App() {
  const chartRef = useRef(null);

  useEffect(() => {
    const chartInstance = echarts.init(chartRef.current);

    // グラフのオプション設定
    const options = {
      title: {
        text: "Sample Line Chart",
      },
      tooltip: {
        trigger: "axis",
      },
      legend: {
        data: ["Dataset 1", "Dataset 2"],
      },
      xAxis: {
        type: "category",
        data: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
      },
      yAxis: {
        type: "value",
      },
      series: [
        {
          name: "Dataset 1",
          type: "line",
          data: [120, 132, 101, 134, 90, 230, 210],
        },
        {
          name: "Dataset 2",
          type: "line",
          data: [220, 182, 191, 234, 290, 330, 310],
        },
      ],
    };

    // オプションを使ってグラフを描画
    chartInstance.setOption(options);

    // クリーンアップ
    return () => {
      chartInstance.dispose();
    };
  }, []);

  return (
    <div>
      <h3>ECharts Line Chart with Legend Toggle</h3>
      <div ref={chartRef} style={{ width: "200%", height: "400px" }} />
    </div>
  );
}

export default App;
