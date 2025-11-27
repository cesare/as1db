-- CreateTable
CREATE TABLE "material_items" (
    "item_id" INTEGER NOT NULL,
    "material_item_id" INTEGER NOT NULL,

    CONSTRAINT "material_items_pkey" PRIMARY KEY ("item_id","material_item_id")
);

-- AddForeignKey
ALTER TABLE "material_items" ADD CONSTRAINT "material_items_item_id_fkey" FOREIGN KEY ("item_id") REFERENCES "items"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "material_items" ADD CONSTRAINT "material_items_material_item_id_fkey" FOREIGN KEY ("material_item_id") REFERENCES "items"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
