-- CreateTable
CREATE TABLE "item_categories" (
    "item_id" INTEGER NOT NULL,
    "category_id" INTEGER NOT NULL,

    CONSTRAINT "item_categories_pkey" PRIMARY KEY ("item_id","category_id")
);

-- AddForeignKey
ALTER TABLE "item_categories" ADD CONSTRAINT "item_categories_item_id_fkey" FOREIGN KEY ("item_id") REFERENCES "items"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "item_categories" ADD CONSTRAINT "item_categories_category_id_fkey" FOREIGN KEY ("category_id") REFERENCES "categories"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
