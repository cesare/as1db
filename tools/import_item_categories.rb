require "csv"
require "pg"

csv_file_name = ARGV[0]

Category = Struct.new(:id, :name)
Item = Struct.new(:id, :name)

db = PG.connect(
  host: "localhost",
  port: 5432,
  user: ENV["POSTGRES_USER"],
  password: ENV["POSTGRES_PASSWORD"],
)

categories = db.exec("select id, name from categories") do |results|
  results.map do |row|
    Category.new(id: row["id"], name: row["name"])
  end
end

items = db.exec("select id, name from items") do |results|
  results.map do |row|
    Item.new(id: row["id"], name: row["name"])
  end
end

CSV.open(csv_file_name, headers: true) do |csv|
  csv.each do |row|
    item_name = row["name"]
    category_name = row["category"]

    category = categories.find { it.name == category_name }
    if category.nil?
      $stderr.puts "category #{category_name} not found"
      next
    end

    item = items.find { it.name == item_name }
    if item.nil?
      $stderr.puts "item #{item_name} not found"
      next
    end

    db.exec("insert into item_categories (item_id, category_id) values ($1, $2)", [item.id, category.id])
    puts "inserted: #{item.name} -> #{category.name}"
  rescue => e
    $stderr.puts "insertion failed: #{e}"
  end
end

db.close
