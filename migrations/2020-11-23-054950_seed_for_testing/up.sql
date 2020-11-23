-- Your SQL goes here
-- add user data
INSERT INTO public.user(email, nama, password) VALUES ('bima@qti.co.id', 'bima', '12qwaszx');
INSERT INTO public.user(email, nama, password) VALUES ('alli@qti.co.id', 'alli', '12qwaszx');

-- add satuan_kerja data
INSERT INTO public.satuan_kerja(nama, kode, deskripsi) VALUES ('BINDA SUMATERA SELATAN', 'BINDA.3.12', 'BINDA SUMATERA SELATAN');
INSERT INTO public.satuan_kerja(nama, kode, deskripsi) VALUES ('BINDA KALIMANTAN TIMUR', 'BINDA.3.8', 'BINDA KALIMANTAN TIMUR');
INSERT INTO public.satuan_kerja(nama, kode, deskripsi) VALUES ('BINDA KEPULAUAN RIAU', 'BINDA.3.9', 'BINDA KEPULAUAN RIAU');
INSERT INTO public.satuan_kerja(nama, kode, deskripsi) VALUES ('BINDA LAMPUNG', 'BINDA.3.10', 'BINDA LAMPUNG');
INSERT INTO public.satuan_kerja(nama, kode, deskripsi) VALUES ('BINDA MALUKU UTARA', 'BINDA.5.3', 'BINDA MALUKU UTARA');

-- alter table referensi
ALTER TABLE referensi 
ADD COLUMN parent_id integer;

-- add referensi data
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (1, 'klasifikasi-laporan', 'Klasifikasi Laporan', 'Kategori Klasifikasi Laporan', NULL);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (2, 'aspek-laporan', 'Aspek Laporan', 'Kategori Aspek Laporan', NULL);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (3, 'jenis-laporan', 'Jenis Laporan', 'Kategori Jenis Laporan', NULL);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (4, 'klasifikasi-rahasia', 'Rahasia', 'Klasifikasi Rahasia pada laporan', 1);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (5, 'klasifikasi-sangat-rahasia', 'Sangat Rahasia', 'Klasifikasi Sangat Rahasia pada Laporan', 1);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (6, 'klasifikasi-terbatas', 'Terbatas', 'Klasifikasi Terbatas pada Laporan', 1);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (7, 'klasifikasi-confidential', 'Confidential', 'Klasifikasi Confidential pada Laporan', 1);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (8, 'lapintel', 'Laporan Intelijen', 'Jenis Laporan Intelijen', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (9, 'lapin', 'Laporan Informasi', 'Jenis Laporan Informasi', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (10, 'lapsit', 'Laporan Situasi', 'Jenis Laporan Situasi', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (11, 'lapat', 'Laporan Atensi', 'Jenis Laporan Atensi', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (12, 'lapbdi', 'BDI', 'Jenis Laporan BDI', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (13, 'lapka', 'Perkiraan Keadaan', 'Jenis Laporan Keadaan', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (14, 'laphar', 'Laporan Harian', 'Jenis Laporan Harian', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (15, 'lapming', 'Laporan Mingguan', 'Jenis Laporan Mingguan', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (16, 'aspek-ideologi', 'Ideologi', 'Aspek Ideologi', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (17, 'aspek-politik', 'Politik', 'Aspek Politik', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (18, 'aspek-ekonomi', 'Ekonomi', 'Aspek Ekonomi', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (19, 'aspek-sosbud', 'Sosial dan Budaya', 'Aspek Sosial dan Budaya', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (20, 'aspek-hankam', 'Pertahanan dan Keamanan', 'Aspek Pertahanan dan Keamanan', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (21, 'aspek-sejarah', 'Sejarah', 'Aspek Sejarah', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (22, 'aspek-iptek', 'Iptek', 'Aspek Iptek', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (23, 'aspek-transportasi', 'Transportasi', 'Aspek Transportasi', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (24, 'aspek-telekomunikasi', 'Telekomunikasi', 'Aspek Telekomunikasi', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (25, 'aspek-biografi', 'Biografi', 'Aspek Biografi', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (26, 'aspek-geografi', 'Geografi', 'Aspek Geografi', 2);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (27, 'jenis-surat', 'Kategori Jenis Surat', 'Kategori Jenis Surat', NULL);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (28, 'uuk', 'UUK', 'UUK', 27);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (29, 'nota-dinas', 'Nota Dinas', 'Nota Dinas', 27);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (30, 'urgensi-laporan', 'Kategori Urgensi Laporan', '', NULL);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (31, 'urgensi-penting', 'Penting', '', 30);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (32, 'urgensi-sangat-penting', 'Sangat Penting', '', 30);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (34, 'memo', 'Memorandum', 'Memorandum', 27);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (41, 'bdi-attribute-tipe', 'Tipe Attribute BDI', 'Tipe Attribute BDI', NULL);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (42, 'text', 'Text', NULL, 41);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (43, 'textarea', 'Textarea', NULL, 41);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (44, 'date', 'Date', NULL, 41);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (45, 'datetime', 'Datetime', NULL, 41);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (46, 'number', 'Number', NULL, 41);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (47, 'hp', 'handphone', '', 24);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (33, 'surat-telegram', 'Surat Telegram / UUK', 'Surat Telegram / UUK', 27);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (48, 'IDEO-KIRI', 'Ideologi Kiri', 'Ideologi Kiri', 16);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (57, 'asdad', 'asdasd', 'asdas', 1);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (71, 'mi', 'Memo Internal', 'Jenis Laporan Memo Internal', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (72, 'br', 'Bahan Rapat', 'Jenis Bahan Rapat', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (63, 'lk', 'Laporan Khusus', 'Laporan', 3);
INSERT INTO public.referensi (id, kode, nama, deskripsi, parent_id) VALUES (56, 'KLA-TST', 'Klasifikasi Testing', 'adv ad2', 1);
