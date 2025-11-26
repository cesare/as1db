require "csv"
require "pg"

csv_file_name = ARGV[0]

Clazz = Struct.new(:id, :name)

db = PG.connect(
  host: "localhost",
  port: 5432,
  user: ENV["POSTGRES_USER"],
  password: ENV["POSTGRES_PASSWORD"],
)

classes = db.exec("select id, name from classes") do |results|
  results.map do |row|
    Clazz.new(id: row["id"], name: row["name"])
  end
end

CSV.open(csv_file_name, headers: true) do |csv|
  csv.each do |row|
    name = row["name"]
    class_name = row["class"]
    clazz = classes.find { it.name == class_name }
    if clazz.nil?
      $stderr.puts "class #{class_name} not found"
      next
    end

    db.exec("insert into items (class_id, name) values ($1, $2)", [clazz.id, name])
    puts "inserted #{name}"
  rescue => e
    $stderr.puts "insertion failed: #{e}"
  end
end

db.close
