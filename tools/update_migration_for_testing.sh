#!/bin/bash

set -euxo pipefail

topdir=$(git rev-parse --show-toplevel)
schema_dir="${topdir}/schema"

backend_dir="${topdir}/backend"
schema_dump_dir="${backend_dir}/migrations"
schema_dump_file="${schema_dump_dir}/schema.sql"

mkdir -p $schema_dump_dir

cd $schema_dir
npx prisma migrate diff --from-empty --to-schema prisma/schema.prisma --script > ${schema_dump_file}
