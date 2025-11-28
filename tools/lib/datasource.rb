require 'pg'

Clazz = Struct.new(:id, :name)
Item = Struct.new(:id, :name)
Category = Struct.new(:id, :name)
Trait = Struct.new(:id, :name)

class DataSet
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
end

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
