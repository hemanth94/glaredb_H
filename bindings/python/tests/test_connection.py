import glaredb
import pytest
import pandas as pd


def test_default_conn_uses_same_db():
    # Create table
    glaredb.execute("create table other (a int)")
    # Try to query it. This would error if we weren't using the same db.
    glaredb.execute("select * from other")
    glaredb.execute("drop table other")


def test_with_context():
    with glaredb.connect() as con:
        con.execute("select 1")


def test_default_connection_sql():
    with glaredb.connect() as db:
        out = db.sql("select 1 as a").to_pandas()
        expected = pd.DataFrame({"a": [1]})
        assert out.equals(expected)


def test_default_connection_execute():
    with glaredb.connect() as db:
        out = db.execute("select 1 as a").to_pandas()
        expected = pd.DataFrame({"a": [1]})
        assert out.equals(expected)
