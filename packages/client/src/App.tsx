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
import { useEffect, useState } from "react";

import * as duckdb from "@duckdb/duckdb-wasm";
import eh_worker from "@duckdb/duckdb-wasm/dist/duckdb-browser-eh.worker.js?url";
import mvp_worker from "@duckdb/duckdb-wasm/dist/duckdb-browser-mvp.worker.js?url";
import duckdb_wasm_eh from "@duckdb/duckdb-wasm/dist/duckdb-eh.wasm?url";
import duckdb_wasm from "@duckdb/duckdb-wasm/dist/duckdb-mvp.wasm?url";

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

const DUCKDB_BUNDLES: duckdb.DuckDBBundles = {
  mvp: {
    mainModule: duckdb_wasm,
    mainWorker: mvp_worker,
  },
  eh: {
    mainModule: duckdb_wasm_eh,
    mainWorker: eh_worker,
  },
};

// ファイルを取得する関数
async function getFileFromS3(filename: string): Promise<string> {
  const domain = "";

  // cloudfront distribution を使っているので、domain は空文字列
  return (await fetch(`${domain}/data/${filename}`)).text();
}

function App() {
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const [result, setResult] = useState<any>(null);
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const [duckdbData, setDuckDBData] = useState<any[]>([]);

  useEffect(() => {
    (async () => {
      const filename = "jma_observation_data_2024_11_1.csv";
      const csvFIleName = filename;

      try {
        const fileContent = await getFileFromS3(filename);
        setResult(fileContent);

        // text to csv file
        new File([fileContent], filename, { type: "text/csv" });

        const bundle = await duckdb.selectBundle(DUCKDB_BUNDLES);
        const logger = new duckdb.ConsoleLogger();
        // biome-ignore lint/style/noNonNullAssertion: <explanation>
        const worker = new Worker(bundle.mainWorker!);
        const db = new duckdb.AsyncDuckDB(logger, worker);
        await db.instantiate(bundle.mainModule);
        const conn = await db.connect();

        // CSVファイルを仮想ファイルとして登録
        await db.registerFileBuffer(
          csvFIleName,
          new TextEncoder().encode(fileContent),
        );

        // 仮想ファイルに対してクエリを実行
        await conn.query(
          `CREATE TABLE amedas_url_list AS SELECT * FROM read_csv_auto('jma_observation_data_2024_11_1.csv')`,
        );
        const duckdbResult = await conn.query("SELECT * FROM amedas_url_list");
        setDuckDBData(duckdbResult.toArray());

        await conn.close();
      } catch (err) {
        console.error("Failed to get file from S3:", err);
        setResult(err);
      }
    })();
  }, []);

  return (
    <>
      <h2> Weather Forecast Comparison</h2>
      <pre style={{ textAlign: "left" }}>
        {JSON.stringify(result, undefined, 2)}
      </pre>
      <table>
        <tbody>
          {duckdbData.map((row, i) => (
            // biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
            <tr key={i}>
              {Object.keys(row).map((key) => (
                <td key={key}>{row[key]}</td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>

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
