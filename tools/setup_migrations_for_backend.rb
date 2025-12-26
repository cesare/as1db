require "fileutils"

topdir = `git rev-parse --show-toplevel`
topdir.chomp!

schema_topdir = "#{topdir}/schema"

backend_topdir = "#{topdir}/backend"
migrations_dir = "#{backend_topdir}/migrations"

Dir.chdir(topdir)
files = Dir.glob("schema/prisma/migrations/**/*.sql")

names = files.map { [File.basename(File.dirname(it)), it] }

FileUtils.mkdir_p(migrations_dir, verbose: true)
Dir.chdir(migrations_dir)
names.each do |name, path|
  target_filename = "#{name}.up.sql"
  if File.exist?(target_filename)
    puts "Skip: #{target_filename} exists"
    next
  end

  src_path = "../../#{path}"
  FileUtils.ln_s(src_path, target_filename, verbose: true)
end
