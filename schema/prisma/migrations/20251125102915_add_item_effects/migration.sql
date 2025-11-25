-- CreateTable
CREATE TABLE "item_effects" (
    "item_id" INTEGER NOT NULL,
    "effect_id" INTEGER NOT NULL,

    CONSTRAINT "item_effects_pkey" PRIMARY KEY ("item_id","effect_id")
);

-- AddForeignKey
ALTER TABLE "item_effects" ADD CONSTRAINT "item_effects_item_id_fkey" FOREIGN KEY ("item_id") REFERENCES "items"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "item_effects" ADD CONSTRAINT "item_effects_effect_id_fkey" FOREIGN KEY ("effect_id") REFERENCES "effects"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
