<p align="center">
  <img src="https://github.com/user-attachments/assets/13ff4c9d-d6a7-4c20-9408-45573c508c41" alt="spice oss logo" width="600"/>
</p>
<p align="center">
  <a href="https://github.com/spiceai/spiceai/actions/workflows/codeql-analysis.yml"><img src="https://github.com/spiceai/spiceai/actions/workflows/codeql-analysis.yml/badge.svg?branch=trunk&event=push" alt="CodeQL"/></a>
  <a href="https://opensource.org/licenses/Apache-2.0"><img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg" alt="License: Apache-2.0"/></a>
  <a href="https://discord.gg/kZnTfneP5u"><img src="https://img.shields.io/discord/803820740868571196" alt="Discord"/></a>
  <a href="https://x.com/intent/follow?screen_name=spice_ai"><img src="https://img.shields.io/twitter/follow/spice_ai.svg?style=social&logo=x" alt="Follow on X"/></a>
</p>

**Spice** is a portable runtime written in Rust that offers developers a unified SQL interface to materialize, accelerate, and query data from any database, data warehouse, or data lake.

📣 Read the [Spice.ai OSS announcement blog post](https://blog.spiceai.org/posts/2024/03/28/adding-spice-the-next-generation-of-spice.ai-oss/).

Spice connects, fuses, and delivers data to applications, machine-learning models, and AI-backends, functioning as an application-specific, tier-optimized Database CDN.

Spice is built-with industry leading technologies such as [Apache DataFusion](https://datafusion.apache.org), Apache Arrow, Apache Arrow Flight, SQLite, and DuckDB.

<div align="center">
  <picture>
    <img width="600" alt="How Spice works." src="https://github.com/spiceai/spiceai/assets/80174/7d93ae32-d6d8-437b-88d3-d64fe089e4b7" />
  </picture>
</div>

🎓 [Read the MaterializedView interview on Spice.ai](https://materializedview.io/p/building-a-cdn-for-databases-spice-ai)

🎥 [Watch the CMU Databases Accelerating Data and AI with Spice.ai Open-Source](https://www.youtube.com/watch?v=tyM-ec1lKfU)

## Why Spice?

Spice makes it fast and easy to query data from one or more sources using SQL. You can co-locate a managed dataset with your application or machine learning model, and accelerate it with Arrow in-memory, SQLite/DuckDB, or with attached PostgreSQL for fast, high-concurrency, low-latency queries. Accelerated engines give you flexibility and control over query cost and performance.

<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/spiceai/spiceai/assets/80174/96b5fcef-a550-4ce8-a74a-83931275e83e">
    <img width="800" alt="Spice.ai" src="https://github.com/spiceai/spiceai/assets/80174/29e4421d-8942-4f2a-8397-e9d4fdeda36b" />
  </picture>
</div>

### How is Spice different?

1. **Application-focused:** Spice is designed to integrate at the application level; 1:1 or 1:N application to Spice mapping, whereas most other data systems are designed for multiple applications to share a single database or data warehouse. It's not uncommon to have many Spice instances, even down to one for each tenant or customer.

2. **Dual-Engine Acceleration:** Spice supports both **OLAP** (Arrow/DuckDB) and **OLTP** (SQLite/PostgreSQL) databases at the dataset level, unlike other systems that only support one type.

3. **Separation of Materialization and Storage/Compute:** Spice separates storage and compute, allowing you to keep data close to its source and bring a materialized working set next to your application, dashboard, or data/ML pipeline.

4. **Edge to Cloud Native**. Spice is designed to be deployed anywhere, from a standalone instance to a Kubernetes container sidecar, microservice, or cluster at the Edge/POP, On-Prem, or in public clouds. You can also chain Spice instances and deploy them across multiple infrastructure tiers.

### How does Spice compare?

|                            | Spice                              | Trino/Presto                     | Dremio                           | Clickhouse              |
| -------------------------- | ---------------------------------- | -------------------------------- | -------------------------------- | ----------------------- |
| Primary Use-Case           | Data & AI Applications             | Big Data Analytics               | Interactive Analytics            | Real-Time Analytics     |
| Typical Deployment         | Colocated with application         | Cloud Cluster                    | Cloud Cluster                    | On-Prem/Cloud Cluster   |
| Application-to-Data System | One-to-One/Many                    | Many-to-One                      | Many-to-One                      | Many-to-One             |
| Query Federation           | Native with query push-down        | Supported with push-down         | Supported with limited push-down | Limited                 |
| Materialization            | Arrow/SQLite/DuckDB/PostgreSQL     | Intermediate Storage             | Reflections (Iceberg)            | Views & MergeTree       |
| Query Result Caching       | Supported                          | Supported                        | Supported                        | Supported               |
| Typical Configuration      | Single-Binary/Sidecar/Microservice | Coodinator+Executor w/ Zookeeper | Coodinator+Executor w/ Zookeeper | Clickhouse Keeper+Nodes |

### Example Use-Cases

**1. Faster applications and frontends.** Accelerate and co-locate datasets with applications and frontends, to serve more concurrent queries and users with faster page loads and data updates. [Try the CQRS sample app](https://github.com/spiceai/samples/tree/trunk/acceleration#local-materialization-and-acceleration-cqrs-sample)

**2. Faster dashboards, analytics, and BI.** Faster, more responsive dashboards without massive compute costs. [Watch the Apache Superset demo](https://github.com/spiceai/samples/blob/trunk/sales-bi/README.md)

**3. Faster data pipelines, machine learning training and inferencing.** Co-locate datasets in pipelines where the data is needed to minimize data-movement and improve query performance.

**4. Easily query many data sources.** Federated SQL query across databases, data warehouses, and data lakes using [Data Connectors](https://docs.spiceai.org/components/data-connectors).

### FAQ

- **Is Spice a cache?** No, however you can think of Spice data materialization like an _active_ cache or data prefetcher. A cache would fetch data on a cache-miss while Spice prefetches and materializes filtered data on an interval or as new data becomes available. In addition to materialization Spice supports [results caching](https://docs.spiceai.org/features/caching).

- **Is Spice a CDN for databases?** Yes, you can think of Spice like a CDN for different data sources. Using CDN concepts, Spice enables you to ship (load) a working set of your database (or data lake, or data warehouse) where it's most frequently accessed, like from a data application or for AI-inference.

- **Where is the AI?** Spice provides a unified API for both data _and_ AI/ML with a high-performance bus between the two. However, because the first step in AI-readiness is data-readiness, the Getting Started content is focused on data. Spice has [endpoints and APIs](https://docs.spiceai.org/api/http) for model deployment and inference including LLMs, accelerated embeddings, and an AI-gateway for providers like OpenAI and Anthropic. Read more about the vision to enable development of [intelligent AI-driven applications](https://docs.spiceai.org/intelligent-applications).

### Watch a 30-sec BI dashboard acceleration demo

<https://github.com/spiceai/spiceai/assets/80174/7735ee94-3f4a-4983-a98e-fe766e79e03a>

### Supported Data Connectors

Currently supported data connectors for upstream datasets. More coming soon.

| Name                               | Description                           | Status            | Protocol/Format              |
| ---------------------------------- | ------------------------------------- | ----------------- | ---------------------------- |
| `duckdb`                           | DuckDB                                | Release Candidate |                              |
| `github`                           | GitHub                                | Release Candidate |                              |
| `graphql`                          | GraphQL                               | Release Candidate | JSON                         |
| `mysql`                            | MySQL                                 | Release Candidate |                              |
| `postgres`                         | PostgreSQL                            | Release Candidate |                              |
| `s3`                               | [S3][s3]                              | Release Candidate | Parquet, CSV                 |
| `databricks (mode: delta_lake)`    | [Databricks][databricks]              | Release Candidate | S3/Delta Lake                |
| `databricks (mode: spark_connect)` | [Databricks][databricks]              | Beta              | [Spark Connect][spark]       |
| `delta_lake`                       | Delta Lake                            | Beta              | Delta Lake                   |
| `file`                             | File                                  | Beta              | Parquet, CSV                 |
| `flightsql`                        | FlightSQL                             | Beta              | Arrow Flight SQL             |
| `mssql`                            | Microsoft SQL Server                  | Beta              | Tabular Data Stream (TDS)    |
| `odbc`                             | ODBC                                  | Beta              | ODBC                         |
| `spiceai`                          | [Spice.ai][spiceai]                   | Beta              | Arrow Flight                 |
| `abfs`                             | Azure BlobFS                          | Alpha             | Parquet, CSV                 |
| `clickhouse`                       | Clickhouse                            | Alpha             |                              |
| `debezium`                         | Debezium CDC                          | Alpha             | Kafka + JSON                 |
| `dremio`                           | [Dremio][dremio]                      | Alpha             | Arrow Flight                 |
| `ftp`, `sftp`                      | FTP/SFTP                              | Alpha             | Parquet, CSV                 |
| `http`, `https`                    | HTTP(s)                               | Alpha             | Parquet, CSV                 |
| `localpod`                         | [Local dataset replication][localpod] | Alpha             |                              |
| `sharepoint`                       | Microsoft SharePoint                  | Alpha             | Unstructured UTF-8 documents |
| `snowflake`                        | Snowflake                             | Alpha             | Arrow                        |
| `spark`                            | Spark                                 | Alpha             | [Spark Connect][spark]       |

[databricks]: https://github.com/spiceai/quickstarts/tree/trunk/databricks#spice-on-databricks
[spark]: https://spark.apache.org/docs/latest/spark-connect-overview.html
[s3]: https://github.com/spiceai/quickstarts/tree/trunk/s3#readme
[spiceai]: https://github.com/spiceai/quickstarts/tree/trunk/spiceai#readme
[dremio]: https://github.com/spiceai/quickstarts/tree/trunk/dremio#readme
[localpod]: https://github.com/spiceai/quickstarts/blob/trunk/localpod/README.md

### Supported Data Stores/Accelerators

Currently supported data stores for local materialization/acceleration. More coming soon.

| Name       | Description                     | Status            | Engine Modes     |
| ---------- | ------------------------------- | ----------------- | ---------------- |
| `arrow`    | In-Memory Arrow Records         | Release Candidate | `memory`         |
| `duckdb`   | Embedded [DuckDB][duckdb]       | Release Candidate | `memory`, `file` |
| `postgres` | Attached [PostgreSQL][postgres] | Beta              |                  |
| `sqlite`   | Embedded [SQLite][sqlite]       | Beta              | `memory`, `file` |

[duckdb]: https://docs.spiceai.org/data-accelerators/duckdb
[postgres]: https://github.com/spiceai/quickstarts/tree/trunk/postgres#postgresql-data-accelerator
[sqlite]: https://docs.spiceai.org/data-accelerators/sqlite

## ⚡️ Quickstart (Local Machine)

<https://github.com/spiceai/spiceai/assets/88671039/85cf9a69-46e7-412e-8b68-22617dcbd4e0>

**Step 1.** Install the Spice CLI:

On **macOS, Linux, and WSL**:

```bash
curl https://install.spiceai.org | /bin/bash
```

Or using `brew`:

```bash
brew install spiceai/spiceai/spice
```

On **Windows**:

```bash
curl -L "https://install.spiceai.org/Install.ps1" -o Install.ps1 && PowerShell -ExecutionPolicy Bypass -File ./Install.ps1
```

**Step 2.** Initialize a new Spice app with the `spice init` command:

```bash
spice init spice_qs
```

A `spicepod.yaml` file is created in the `spice_qs` directory. Change to that directory:

```bash
cd spice_qs
```

**Step 3.** Start the Spice runtime:

```bash
spice run
```

Example output will be shown as follows:

```bash
Spice.ai runtime starting...
2024-08-05T13:02:40.247484Z  INFO runtime::flight: Spice Runtime Flight listening on 127.0.0.1:50051
2024-08-05T13:02:40.247490Z  INFO runtime::metrics_server: Spice Runtime Metrics listening on 127.0.0.1:9090
2024-08-05T13:02:40.247949Z  INFO runtime: Initialized results cache; max size: 128.00 MiB, item ttl: 1s
2024-08-05T13:02:40.248611Z  INFO runtime::http: Spice Runtime HTTP listening on 127.0.0.1:8090
2024-08-05T13:02:40.252356Z  INFO runtime::opentelemetry: Spice Runtime OpenTelemetry listening on 127.0.0.1:50052
```

The runtime is now started and ready for queries.

**Step 4.** In a new terminal window, add the `spiceai/quickstart` Spicepod. A Spicepod is a package of configuration defining datasets and ML models.

```bash
spice add spiceai/quickstart
```

The `spicepod.yaml` file will be updated with the `spiceai/quickstart` dependency.

```yaml
version: v1beta1
kind: Spicepod
name: spice_qs
dependencies:
  - spiceai/quickstart
```

The `spiceai/quickstart` Spicepod will add a `taxi_trips` data table to the runtime which is now available to query by SQL.

```bash
2024-08-05T13:04:56.742779Z  INFO runtime: Dataset taxi_trips registered (s3://spiceai-demo-datasets/taxi_trips/2024/), acceleration (arrow, 10s refresh), results cache enabled.
2024-08-05T13:04:56.744062Z  INFO runtime::accelerated_table::refresh_task: Loading data for dataset taxi_trips
2024-08-05T13:05:03.556169Z  INFO runtime::accelerated_table::refresh_task: Loaded 2,964,624 rows (421.71 MiB) for dataset taxi_trips in 6s 812ms.
```

**Step 5.** Start the Spice SQL REPL:

```bash
spice sql
```

The SQL REPL inferface will be shown:

```bash
Welcome to the Spice.ai SQL REPL! Type 'help' for help.

show tables; -- list available tables
sql>
```

Enter `show tables;` to display the available tables for query:

```bash
sql> show tables;
+---------------+--------------+---------------+------------+
| table_catalog | table_schema | table_name    | table_type |
+---------------+--------------+---------------+------------+
| spice         | public       | taxi_trips    | BASE TABLE |
| spice         | runtime      | query_history | BASE TABLE |
| spice         | runtime      | metrics       | BASE TABLE |
+---------------+--------------+---------------+------------+

Time: 0.022671708 seconds. 3 rows.
```

Enter a query to display the longest taxi trips:

```sql
SELECT trip_distance, total_amount FROM taxi_trips ORDER BY trip_distance DESC LIMIT 10;
```

Output:

```bash
+---------------+--------------+
| trip_distance | total_amount |
+---------------+--------------+
| 312722.3      | 22.15        |
| 97793.92      | 36.31        |
| 82015.45      | 21.56        |
| 72975.97      | 20.04        |
| 71752.26      | 49.57        |
| 59282.45      | 33.52        |
| 59076.43      | 23.17        |
| 58298.51      | 18.63        |
| 51619.36      | 24.2         |
| 44018.64      | 52.43        |
+---------------+--------------+

Time: 0.045150667 seconds. 10 rows.
```

## ⚙️ Runtime Container Deployment

Using the [Docker image](https://hub.docker.com/r/spiceai/spiceai) locally:

```bash
docker pull spiceai/spiceai
```

In a Dockerfile:

```dockerfile
from spiceai/spiceai:latest
```

Using Helm:

```bash
helm repo add spiceai https://helm.spiceai.org
helm install spiceai spiceai/spiceai
```

## 🏎️ Next Steps

You can use any number of predefined datasets available from the Spice.ai Cloud Platform in the Spice runtime.

A list of publicly available datasets from Spice.ai can be found here: [https://docs.spice.ai/building-blocks/datasets](https://docs.spice.ai/building-blocks/datasets).

In order to access public datasets from Spice.ai, you will first need to create an account with Spice.ai by selecting the free tier membership.

Navigate to [spice.ai](https://spice.ai/) and create a new account by clicking on Try for Free.

<img width="500" alt="spiceai_try_for_free-1" src="https://github.com/spiceai/spiceai/assets/112157037/27fb47ed-4825-4fa8-94bd-48197406cfaa">

After creating an account, you will need to create an app in order to create to an API key.

![create_app-1](https://github.com/spiceai/spiceai/assets/112157037/d2446406-1f06-40fb-8373-1b6d692cb5f7)

You will now be able to access datasets from Spice.ai. For this demonstration, we will be using the `taxi_trips` dataset from the https://spice.ai/spiceai/quickstart Spice.ai app.

**Step 1.** Initialize a new project.

```bash
# Initialize a new Spice app
spice init spice_app

# Change to app directory
cd spice_app
```

**Step 2.** Log in and authenticate from the command line using the `spice login` command. A pop up browser window will prompt you to authenticate:

```bash
spice login
```

**Step 3.** Start the runtime:

```bash
# Start the runtime
spice run
```

**Step 4.** Configure the dataset:

In a new terminal window, configure a new dataset using the `spice dataset configure` command:

```bash
spice dataset configure
```

Enter a dataset name that will be used to reference the dataset in queries. This name does not need to match the name in the dataset source.

```bash
dataset name: (spice_app) taxi_trips
```

Enter the description of the dataset:

```bash
description: Taxi trips dataset
```

Enter the location of the dataset:

```bash
from: spice.ai/spiceai/quickstart/datasets/taxi_trips
```

Select `y` when prompted whether to accelerate the data:

```bash
Locally accelerate (y/n)? y
```

You should see the following output from your runtime terminal:

```bash
2024-12-16T05:12:45.803694Z  INFO runtime::init::dataset: Dataset taxi_trips registered (spice.ai/spiceai/quickstart/datasets/taxi_trips), acceleration (arrow, 10s refresh), results cache enabled.
2024-12-16T05:12:45.805494Z  INFO runtime::accelerated_table::refresh_task: Loading data for dataset taxi_trips
2024-12-16T05:13:24.218345Z  INFO runtime::accelerated_table::refresh_task: Loaded 2,964,624 rows (8.41 GiB) for dataset taxi_trips in 38s 412ms.
```

**Step 5.** In a new terminal window, use the Spice SQL REPL to query the dataset

```bash
spice sql
```

```bash
SELECT tpep_pickup_datetime, passenger_count, trip_distance from taxi_trips LIMIT 10;
```

The output displays the results of the query along with the query execution time:

```bash
+----------------------+-----------------+---------------+
| tpep_pickup_datetime | passenger_count | trip_distance |
+----------------------+-----------------+---------------+
| 2024-01-11T12:55:12  | 1               | 0.0           |
| 2024-01-11T12:55:12  | 1               | 0.0           |
| 2024-01-11T12:04:56  | 1               | 0.63          |
| 2024-01-11T12:18:31  | 1               | 1.38          |
| 2024-01-11T12:39:26  | 1               | 1.01          |
| 2024-01-11T12:18:58  | 1               | 5.13          |
| 2024-01-11T12:43:13  | 1               | 2.9           |
| 2024-01-11T12:05:41  | 1               | 1.36          |
| 2024-01-11T12:20:41  | 1               | 1.11          |
| 2024-01-11T12:37:25  | 1               | 2.04          |
+----------------------+-----------------+---------------+

Time: 0.00538925 seconds. 10 rows.
```

You can experiment with the time it takes to generate queries when using non-accelerated datasets. You can change the acceleration setting from `true` to `false` in the datasets.yaml file.

### 📄 Documentation

Comprehensive documentation is available at [docs.spiceai.org](https://docs.spiceai.org/).

### 🔌 Extensibility

Spice.ai is designed to be extensible with extension points documented at [EXTENSIBILITY.md](./docs/EXTENSIBILITY.md). Build custom [Data Connectors](https://docs.spiceai.org/components/data-connectors), [Data Accelerators](https://docs.spiceai.org/components/data-accelerators), [Catalog Connectors](https://docs.spiceai.org/components/catalogs), [Secret Stores](https://docs.spiceai.org/components/secret-stores), [Models](https://docs.spiceai.org/components/models), or [Embeddings](https://docs.spiceai.org/components/embeddings).

### 🔨 Upcoming Features

🚀 See the [Roadmap to v1.0-stable](https://github.com/spiceai/spiceai/blob/trunk/docs/ROADMAP.md) for upcoming features.

### 🤝 Connect with us

We greatly appreciate and value your support! You can help Spice in a number of ways:

- Build an app with Spice.ai and send us feedback and suggestions at [hey@spice.ai](mailto:hey@spice.ai) or on [Discord](https://discord.gg/kZnTfneP5u), [X](https://twitter.com/spice_ai), or [LinkedIn](https://www.linkedin.com/company/74148478).
- [File an issue](https://github.com/spiceai/spiceai/issues/new) if you see something not quite working correctly.
- Join our team ([We’re hiring!](https://spice.ai/careers))
- Contribute code or documentation to the project (see [CONTRIBUTING.md](CONTRIBUTING.md)).
- Follow our blog at [blog.spiceai.org](https://blog.spiceai.org)

⭐️ star this repo! Thank you for your support! 🙏
