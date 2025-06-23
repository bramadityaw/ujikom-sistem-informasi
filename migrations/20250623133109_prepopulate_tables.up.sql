insert into divisi (kode, nama, pimpinan) values
    ("S1", "Gudang", "Beni Permana, SE"),
    ("S2", "Produksi", "Syaiful Bachri, ST"),
    ("S3", "HRD", "Dr. Anggit Darmawan");

insert into pegawai (nip, nama, alamat, tanggal_lahir, kode_divisi) values
    (11234, "Arini Nikita", "Jl. Dedali Putih 5A Tangerang", "02-Jan-88", "S1"),
    (11235, "Toni Purana", "Jl. Tumenggung 21B Jakarta Selatan", "04-Apr-83", "S2"),
    (11236, "Gigih Prayitno", "Jl. Sidopekso 65 Bandung", "08-Nov-85", "S3"),
    (11237, "Hilda Rahmawa", "Jl. Mendut 12 Bogor", "01-Nov-86", "S2"),
    (11238, "Miftahul Fauza", "Jl. Borobudur 1 Bogor", "05-Sep-84", "S1"),
    (11239, "Katrilia Tirta", "Jl. Kenanga 21 Jakarta Timur", "05-Jul-84", "S1");

insert into presensi (tanggal, nip, jam_masuk, jam_pulang) values
    ("02-Jan-18", 11234, "8:10", "17:40"),
    ("02-Jan-18", 11235, "8:00", "17:07"),
    ("02-Jan-18", 11236, "7:00", "16:30"),
    ("03-Jan-18", 11234, "7:45", "16:40"),
    ("03-Jan-18", 11235, "7:50", "16:50"),
    ("04-Jan-18", 11234, "7:55", "17:30"),
    ("05-Jan-18", 11234, "7:20", "16:20");
