require 'pg'

Clazz = Struct.new(:id, :name)
Item = Struct.new(:id, :name)
Category = Struct.new(:id, :name)
Trait = Struct.new(:id, :name)

class DataSet
  include Enumerable

  private attr_reader :records

  def initialize(records)
    @records = records
  end

  def find_by_id(id)
    records.find { it.id == id }
  end

  def find_by_name(name)
    records.find { it.name == name }
  end

  def each(&block) = records.each(&block)
end

ItemCategory = Struct.new(:item, :category)
MaterialItem = Struct.new(:item, :material)
MaterialCategory = Struct.new(:item, :category)

class DataSource
  private attr_reader :db

  def initialize
    @db = connect
  end

  def close
    db.close
  end

  def classes
    records = db.exec("select id, name from classes") do |results|
      results.map do |row|
        Clazz.new(id: row["id"].to_i, name: row["name"])
      end
    end
    DataSet.new(records)
  end

  def items
    records = db.exec("select id, name from items") do |results|
      results.map do |row|
        Item.new(id: row["id"].to_i, name: row["name"])
      end
    end
    DataSet.new(records)
  end

  def categories
    records = db.exec("select id, name from categories") do |results|
      results.map do |row|
        Category.new(id: row["id"].to_i, name: row["name"])
      end
    end
    DataSet.new(records)
  end

  def traits
    records = db.exec("select id, name from traits") do |results|
      results.map do |row|
        Trait.new(id: row["id"].to_i, name: row["name"])
      end
    end
    DataSet.new(records)
  end

  def items_with_class
    statement = <<~SQL
      select
        items.id as item_id,
        items.name as item_name,
        classes.id as class_id,
        classes.name as class_name
      from items
        inner join classes on items.class_id = classes.id
      order by 1, 3
    SQL

    db.exec(statement) do |results|
      results.map do |row|
        item = Item.new(row["item_id"].to_i, row["item_name"])
        clazz = Clazz.new(row["class_id"].to_i, row["class_name"])
        [item, clazz]
      end
    end
  end

  def item_categories
    statement = <<~SQL
      select
        items.id as item_id,
        items.name as item_name,
        categories.id as category_id,
        categories.name as category_name
      from item_categories
        inner join items on item_categories.item_id = items.id
        inner join categories on item_categories.category_id = categories.id
      order by 1, 3
    SQL

    db.exec(statement) do |results|
      results.map do |row|
        item = Item.new(row["item_id"].to_i, row["item_name"])
        category = Category.new(row["category_id"].to_i, row["category_name"])
        ItemCategory.new(item, category)
      end
    end
  end

  def material_items
    statement = <<~SQL
      select
        item.id as item_id,
        item.name as item_name,
        material.id as material_item_id,
        material.name as material_item_name
      from material_items as mi
        inner join items as item on mi.item_id = item.id
        inner join items as material on mi.material_item_id = material.id
      order by 1, 3
    SQL

    db.exec(statement) do |results|
      results.map do |row|
        item = Item.new(row["item_id"].to_i, row["item_name"])
        material = Item.new(row["material_item_id"].to_i, row["material_item_name"])
        MaterialItem.new(item, material)
      end
    end
  end

  def material_categories
    statement = <<~SQL
      select
        items.id as item_id,
        items.name as item_name,
        categories.id as category_id,
        categories.name as category_name
      from material_categories as mc
        inner join items on mc.item_id = items.id
        inner join categories on mc.category_id = categories.id
      order by 1, 3
    SQL

    db.exec(statement) do |results|
      results.map do |row|
        item = Item.new(row["item_id"].to_i, row["item_name"])
        category = Category.new(row['category_id'].to_i, row['category_name'])
        MaterialCategory.new(item, category)
      end
    end
  end

  private

  def connect
    PG.connect(
      host: "localhost",
      port: 5432,
      user: ENV["POSTGRES_USER"],
      password: ENV["POSTGRES_PASSWORD"],
    )
  end
end
