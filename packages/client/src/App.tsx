import {
  CartesianGrid,
  Legend,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import "./App.css";
import { useEffect, useRef, useState } from "react";

import * as duckdb from "@duckdb/duckdb-wasm";
import eh_worker from "@duckdb/duckdb-wasm/dist/duckdb-browser-eh.worker.js?url";
import mvp_worker from "@duckdb/duckdb-wasm/dist/duckdb-browser-mvp.worker.js?url";
import duckdb_wasm_eh from "@duckdb/duckdb-wasm/dist/duckdb-eh.wasm?url";
import duckdb_wasm from "@duckdb/duckdb-wasm/dist/duckdb-mvp.wasm?url";

import type { Int } from "apache-arrow";

const data = [
  {
    name: "Page A",
    uv: 4000,
    pv: 2400,
    amt: 2400,
  },
  {
    name: "Page B",
    uv: 3000,
    pv: 1398,
    amt: 2210,
  },
  {
    name: "Page C",
    uv: 2000,
    pv: 9800,
    amt: 2290,
  },
  {
    name: "Page D",
    uv: 2780,
    pv: 3908,
    amt: 2000,
  },
  {
    name: "Page E",
    uv: 1890,
    pv: 4800,
    amt: 2181,
  },
  {
    name: "Page F",
    uv: 2390,
    pv: 3800,
    amt: 2500,
  },
  {
    name: "Page G",
    uv: 3490,
    pv: 4300,
    amt: 2100,
  },
];

const MANUAL_BUNDLES: duckdb.DuckDBBundles = {
  mvp: {
    mainModule: duckdb_wasm,
    mainWorker: mvp_worker,
  },
  eh: {
    mainModule: duckdb_wasm_eh,
    mainWorker: eh_worker,
  },
};

const DUCKDB_BUNDLES: duckdb.DuckDBBundles = {
  mvp: {
    mainModule: duckdb_wasm,
    mainWorker: new URL(
      "@duckdb/duckdb-wasm/dist/duckdb-browser-mvp.worker.js",
      import.meta.url,
    ).toString(),
  },
  eh: {
    mainModule: duckdb_wasm_eh,
    mainWorker: new URL(
      "@duckdb/duckdb-wasm/dist/duckdb-browser-eh.worker.js",
      import.meta.url,
    ).toString(),
  },
};

function App() {
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const [result, setResult] = useState<any>(null);

  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    setResult(file);
    if (file) {
      const reader = new FileReader();
      reader.onload = async () => {
        const csvText = reader.result as string;

        const bundle = await duckdb.selectBundle(DUCKDB_BUNDLES);
        const logger = new duckdb.ConsoleLogger();
        // biome-ignore lint/style/noNonNullAssertion: <explanation>
        const worker = new Worker(bundle.mainWorker!);
        const db = new duckdb.AsyncDuckDB(logger, worker);
        await db.instantiate(bundle.mainModule);
        const conn = await db.connect();

        // CSVファイルを仮想ファイルとして登録
        await db.registerFileBuffer(
          "amedas_url_list1.csv",
          new TextEncoder().encode(csvText),
        );

        // 仮想ファイルに対してクエリを実行
        await conn.query(
          `CREATE TABLE amedas_url_list AS SELECT * FROM read_csv_auto('amedas_url_list1.csv')`,
        );
        const duckdbResult = await conn.query("SELECT * FROM amedas_url_list");
        setResult(duckdbResult.batches[0].data.children);

        await conn.close();
      };
      reader.readAsText(file);
    }
  };

  return (
    <>
      <h2> Weather Forecast Comparison</h2>
      <input type="file" accept=".csv" onChange={handleFileUpload} />
      {/* <p>レンダリング回数: {renderCount.current}</p>{" "} */}
      {/* レンダリング回数を表示 */}
      <pre style={{ textAlign: "left" }}>
        {JSON.stringify(result, undefined, 2)}
      </pre>
      <ResponsiveContainer width={"101%"} height={300}>
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="name" padding={{ left: 30, right: 30 }} />
          <YAxis />
          <Tooltip />
          <Legend />
          <Line
            type="monotone"
            dataKey="pv"
            stroke="#8884d8"
            activeDot={{ r: 8 }}
          />
          <Line type="monotone" dataKey="uv" stroke="#82ca9d" />
        </LineChart>
      </ResponsiveContainer>
    </>
  );
}

export default App;
