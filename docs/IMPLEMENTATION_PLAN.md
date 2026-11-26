# Snowflake driver implementation plan

## 1. Prove authentication and connection context

Start with a draft PR implementing `test_connection`. Evaluate the
[Snowflake SQL API](https://docs.snowflake.com/en/developer-guide/sql-api)
with OAuth against a dedicated account. Record token provisioning, expiry
behavior and required privileges. Confirm which host credential fields can
store the token and which per-connection fields carry the HTTPS account
endpoint, role, warehouse, database and schema. Keep secrets out of logs,
examples and repository fixtures.

Use a mock HTTP server in CI. In the live smoke test, query the current role,
warehouse, database and schema. Switch between two saved connections and
verify all four values each time. Cover expired tokens and insufficient grants.

## 2. Browse role-visible metadata

Implement database, schema, table, column and view discovery. Seed a fixture
with a regular table, a view, a reserved word and a quoted mixed-case name.
Verify exact names, types and database/schema qualification. Repeat under a
restricted role to check visibility. Enable each manifest capability alongside
its tested handler.

## 3. Run and cancel queries

Implement statement submission, asynchronous status polling and result
partition retrieval. Keep the statement handle with its initiating connection.
Test a multi-partition result for missing or repeated rows, including a page
boundary inside a partition. Verify table browsing limits and offsets. Exercise
timeouts, server cancellation and failures while polling. Run a query in both
the editor and a notebook and confirm the selected execution context.

## 4. Preserve Snowflake values

Add fixtures for NULL, a 38-digit NUMBER, fractional decimals, binary data,
DATE, TIME, TIMESTAMP_NTZ/LTZ/TZ and nested VARIANT, OBJECT and ARRAY values.
Compare rendered values with the seed SQL and document how precision and time
zones are represented in Tabularis. Keep grid edits, DDL and Visual EXPLAIN
disabled until each has its own tested implementation.

## 5. Qualify the first release

Commit seed and cleanup SQL plus an opt-in live-account test recipe. Record the
tested account configuration, auth method and minimum role grants. Verify
installation from the packaged archives, then publish a prerelease and submit
the tested manifest to the registry. Update the main issue and bounty board
with the release and any remaining limitations.
