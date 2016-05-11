<html>
  <head>
  </head>
  <body>
    <?php
    $json_file = file_get_contents('current.json');
    // convert the string to a json object
    $jfo = json_decode($json_file);
    foreach ($jfo as $run) {
      echo "<p>[" . $run->Language . "] " . $run->Variant . ": " . $run->GFLOPS_s ." GFLOPS/s</p>";
    }
    ?>
  </body>
</html>
