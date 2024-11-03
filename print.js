const { argv, stdout } = process;

for (const arg of argv.slice(2)) {
  stdout.write(arg.toUpperCase());
  stdout.write("\n");
}
