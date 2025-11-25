-- CreateTable
CREATE TABLE "item_traits" (
    "item_id" INTEGER NOT NULL,
    "trait_id" INTEGER NOT NULL,

    CONSTRAINT "item_traits_pkey" PRIMARY KEY ("item_id","trait_id")
);

-- AddForeignKey
ALTER TABLE "item_traits" ADD CONSTRAINT "item_traits_item_id_fkey" FOREIGN KEY ("item_id") REFERENCES "items"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "item_traits" ADD CONSTRAINT "item_traits_trait_id_fkey" FOREIGN KEY ("trait_id") REFERENCES "traits"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
