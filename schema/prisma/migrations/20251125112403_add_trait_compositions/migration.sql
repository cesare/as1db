-- CreateTable
CREATE TABLE "trait_compositions" (
    "trait_id" INTEGER NOT NULL,
    "lhs_id" INTEGER NOT NULL,
    "rhs_id" INTEGER NOT NULL,

    CONSTRAINT "trait_compositions_pkey" PRIMARY KEY ("trait_id")
);

-- CreateIndex
CREATE UNIQUE INDEX "trait_compositions_lhs_id_rhs_id_key" ON "trait_compositions"("lhs_id", "rhs_id");

-- AddForeignKey
ALTER TABLE "trait_compositions" ADD CONSTRAINT "trait_compositions_trait_id_fkey" FOREIGN KEY ("trait_id") REFERENCES "traits"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "trait_compositions" ADD CONSTRAINT "trait_compositions_lhs_id_fkey" FOREIGN KEY ("lhs_id") REFERENCES "traits"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "trait_compositions" ADD CONSTRAINT "trait_compositions_rhs_id_fkey" FOREIGN KEY ("rhs_id") REFERENCES "traits"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
