/*
Bicycle is a framework for managing data.

Copyright (C) 2024 Ordinary Labs

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

if (!globalThis.Bicycle) globalThis.Bicycle = {};

globalThis.Bicycle = {
    ...globalThis.Bicycle,

    get_examples_by_pk: (indexQuery) => Deno.core.ops.op_get_examples_by_pk(indexQuery),
    delete_examples_by_pk: (indexQuery) => Deno.core.ops.op_delete_examples_by_pk(indexQuery),
    put_example: (example) => Deno.core.ops.op_put_example(example),
    batch_put_example: (examples) => Deno.core.ops.op_batch_put_examples(examples),
};