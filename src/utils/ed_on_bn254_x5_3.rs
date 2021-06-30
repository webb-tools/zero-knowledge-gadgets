// sage generate_parameters_grain.sage 1 0 254 3 8 57
// 0x60c89ce5c263405370a08b6d0302b0bab3eedb83920ee0a677297dc392126f1
pub const ROUND_CONSTS: [&str; 195] = [
	"0x00f1445235f2148c5986587169fc1bcd887b08d4d00868df5696fff40956e864",
	"0x0319d062072bef7ecca5eac06f97d4d55952c175ab6b03eae64b44c7dbf11cfa",
	"0x04df5a56ff95bcafb051f7b1cd43a99ba731ff67e47032058fe3d4185697cc7d",
	"0x052cba2255dfd00c7c483143ba8d469448e43586a9b4cd9183fd0e843a6b9fa6",
	"0x03150b7cd6d5d17b2529d36be0f67b832c4acfc884ef4ee5ce15be0bfb4a8d09",
	"0x005032551e6378c450cfe129a404b3764218cadedac14e2b92d2cd73111bf0f9",
	"0x05c8f4f4ebd4a6e3c980d31674bfbe6323037f21b34ae5a4e80c2d4c24d60280",
	"0x037c2849e191ca3edb1c5e49f6e8b8917c843e379366f2ea32ab3aa88d7f8448",
	"0x05a6811f8556f014e92674661e217e9bd5206c5c93a07dc145fdb176a716346f",
	"0x028a305847c683f646fca925c163ff5ae74f348d62c2b670f1426cef9403da53",
	"0x0081c95bc43384e663d79270c956ce3b8925b4f6d033b078b96384f50579400e",
	"0x00ef653322b13d6c889bc81715c37d77a6cd267d595c4a8909a5546c7c97cff1",
	"0x04c6187e41ed881dc1b239c88f7f9d43a9f52fc8c8b6cdd1e76e47615b51f100",
	"0x01a5c536273c2d9df578bfbd32c17b7a2ce3664c2a52032c9321ceb1c4e8a8e4",
	"0x041294d2cc484d228f5784fe7919fd2bb925351240a04b711514c9c80b65af1d",
	"0x04f6eeca1751f7308ac59eff5beb261e4bb563583ede7bc92a738223d6f76e13",
	"0x05a8c4f9968b8aa3b7b478a30f9a5b63650f19a75e7ce11ca9fe16c0b76c00bc",
	"0x04a12ededa9dfd689672f8c67fee31636dcd8e88d01d49019bd90b33eb33db69",
	"0x002e6f8d6520cd4713e335b8c0b6d2e647e9a98e12f4cd2558828b5ef6cb4c9b",
	"0x00b9831b948525595ee02724471bcd182e9521f6b7bb68f1e93be4febb0d3cbe",
	"0x00248156142fd0373a479f91ff239e960f599ff7e94be69b7f2a290305e1198d",
	"0x0171eb95dfbf7d1eaea97cd385f780150885c16235a2a6a8da92ceb01e504233",
	"0x01ca8be73832b8d0681487d27d157802d741a6f36cdc2a0576881f9326478875",
	"0x04e1181763050e58013444dbcb99f1902b11bc25d90bbdca408d3819f4fed32b",
	"0x058cbe8a9a5027bdaa4efb623adead6275f08686f1c08984a9d7c5bae9b4f1c0",
	"0x03464990f045c6ee0819ca51fd11b0be7f61b8eb99f14b77e1e6634601d9e8b5",
	"0x04ef51591c6ead97ef42f287adce40d93abeb032b922f66ffb7e9a5a7450544d",
	"0x054efa1f65b0fce283808965275d877b438da23ce5b13e1963798cb1447d25a4",
	"0x01d660053722ca750a566e73233a0d6e61b31635b817de1930fa1e25771900ef",
	"0x0337ee46060bc1aecb7a640e2a848282231e8827f0aa66f6983a41090014115b",
	"0x0379365e334dd2e444424ec2f797266837458133138811067ea3e48a5f1eb4e6",
	"0x0609e74e70a60d1e1c171f3a2ef2a5e77bab92329f4082404f48845de02248da",
	"0x00cbfb53e6439bc9e48901cc5128c2111ff709502ccfe051b39c95a6fef728ca",
	"0x04212464f8f4bdf80b28a3d00a5f7145679665cdef546a1bbc3f25878fcc4893",
	"0x060aca1b94630aa5f212c2d2d47d9f1dd185c3aa0eaea9e4747d1ac209ad2f6c",
	"0x030662113e0d53904fe77b63efeea42559cdaacbf6662d4517aaee75124335ff",
	"0x03c2d7de3754002820a904765b20fb66e277e73a11202aa55878b5efaba5f1b4",
	"0x0096a9bd16da255009b41dee1c635852606c78fee239d27261cffcb7c9673d7d",
	"0x00ca04139b23c47f7b9debbf08bbbb9bde7757312defb9d59d05bdd655f367a1",
	"0x03c41557aba03f4495d5c7ee7cf0e57e36917fa064d52d284e119e07e194a31e",
	"0x051c2e94f38f6f5514d68e67cd4d3b165484d36a8f3e931af2ab2ade784b463f",
	"0x05b50530fb451875d3de0f282b86fdc816b9ba0b4696c48ea506e5588b36fe64",
	"0x01e5ae8c85a87af1041fa9c99807425c91322aebf169575ca613c0def427401e",
	"0x018dcc0010ca4b5ea304305dbc5c618527cad2635c6e2f84f43050e2e75901ef",
	"0x05054e4a9bace905ab1ac2038269a61cf0289ed9cc4a212e2139434d9966c2d1",
	"0x010ee17c28be66018fc7adf054419043f2c5710927708b5d98e703c6cc8a7440",
	"0x03e590ba5b8b68acbf8beee54ea740960cccfdea998596de1c4b6b92a20577b7",
	"0x03d0f183ce466d5b19339520a4942d1d5de900a1157921049c3fa155be3cb75e",
	"0x05d8f8a9613cc87cd655d21b09ec4c17f9dd40db8878ca9ca9336954ce8124ce",
	"0x04f95707f00ce8a7bb4a30be50fb0b04952292ee5687f625b0484944c1587944",
	"0x04e819e7ca7ab59cf5240b48b647f932263d0cdecd2b3ee2a03e625e4d30bd4b",
	"0x04a5e61dbf56c7dbdf00998730706c815c11aa5f01fbe30b895bffac1d6b9deb",
	"0x026fb79075e52ceec3ea552b02d3d9f6a3e5afeb7583155042f548b72959eb88",
	"0x0483dc085e2b600ea240450a4e2ed49db49466e89536ca6e92c4381e37da7231",
	"0x010b45b94a52951a5c03ef42bbb534b9fc98bf02937ee5ea0ffc27bffca20f8a",
	"0x03b838b42542c71b3067163c455577067a80840aaac1032de40d2b1d1d448e8e",
	"0x036f45bcee6915fe62ea1d494902cfd4126e6834f2782581899a8be9dfc8ac55",
	"0x05aa45495d830fa40696b21603e418f3fe523373b6b9c79ab4f2be0455e6aa90",
	"0x055771c716cb62e937bad9fec7d13a08c273522a7bb9a9f4dec8f49e1c2c86b6",
	"0x02a6e4ee0fe6d4d1bf62816e919ff5de657957b8dbd6ff018129c4523b5e9be7",
	"0x02a6d66f8c72ea2c7c3c29a0e615e0bfbc5e2185456ad57a6567c0fcb4300c9f",
	"0x022cdefdf39a02ffb7acb164b37ca603ef19faaca905b61717c40c23551c8d51",
	"0x0258906cd21b3b0a62d685b57c11a6d8c25e95c4894f8541d5debef009786583",
	"0x0266a56301735625d39cb4be81bf4f3d832f3c01aa2b1046306ebd7ae4f687d5",
	"0x05618810d9da5aacf76808809f616527ef5ab726432c42d945e75944ab36dc55",
	"0x05dab644eecfc4a16fbe7a86a5b1497dc60c5072d9b8d8ce757dd55a955bef01",
	"0x04dfadcdd6f6f7e0b39ee85f4c31f0e5fa57189be933be87c328a69a997a5a62",
	"0x00c4acf1b7ccc766469f5140ced1b684b87ca347648916159fef0192dfd5f801",
	"0x011c8ed279cb5f40b3e172b0c67dfa5d1be5aa0e70232cdcdef77ebe4e7c129a",
	"0x049ef8d75e78cfd5512b700716bbadd6fd6e8a84e21395c4d7372fe703b71413",
	"0x04bb5b1b04d535165d48b08a32ec59aaa196d76ab15ebddd0b39ce6b600221d0",
	"0x05dce4f11bf30ed5cd8ec7c3fcd51c99203ab6ef919932689e5e575100859eca",
	"0x04a0c37073ab91edf7a0c1d10538c54dd6ed10ea907d837749da7204bed8ab78",
	"0x034b0b3ad4fb25a4ef52a58f5fc4d0b30209176259ece9c3da8912c55fa0d835",
	"0x0067a10ea59e58774eaa3854218693262cba96f7dbd98b053a2f935be9a39973",
	"0x00fd469200625b32a580e794d5d955512cdb201dbf71dfc1ac2cdefad40363e1",
	"0x02dbf683d272155aadd053f79ed3dfeacc0230674806a6504ff397bab13dff48",
	"0x00a4832a89079d223e99b6d348f2f8ccb366c659b82fc4b170a0fc78e467f88c",
	"0x03c6dc7c80b9ec64ba8202d69e13edd2098cde8faa1b6cf274c5efdee7ba3128",
	"0x050623ca16463e01017fb545c2e3435a07128a4e08eb1e5a5f3b250a64c3e7e7",
	"0x0391d6051b37c381518229d617dc0bb8d798866d71fe48982e48f60638328403",
	"0x03ceb411203346ee7d4dc1832bd4a58aa6f03b1d2c0c7514ab852f95fc581a2f",
	"0x0007c7a3374e27916b10c1b8ceb93cdda49b79f8b6a877e7c9ee0551da34c577",
	"0x02d24eba783795883dd7803b6abed6a9e8505b7d101769686d5a8355af743762",
	"0x04a41c78ee35e64c5ec7e4e7638317edd8a189d8b420a66af74bc2121e6caad4",
	"0x023cef064aa6ba6f7304539893c02edc5ddcdeaf1900ede1342eb5a96af8c725",
	"0x01a46abbc259f59ab07b81c12f514753eb8b783e7342bffb94a583c3ecf091ed",
	"0x00713ebb6609c8b551591ecc6012f1b4c1a8578e4c794e27ecd41e46e1cf630f",
	"0x031fa138e9346790ec9df4c938e083c03bf0ec6fbfb71e5a353c3151e727df05",
	"0x012c9dd548bd269e256679ba2682e8a5dd7b930e7b32c01c575d3fd0e94feb7e",
	"0x05ccf12176ff95af07f7e7b51bddede9e4c028aa553a5fb47fe5fdd6db574658",
	"0x028c362dcfcf0e8da95dbb8c9bdeb9169a4df06f67f395f5d079fec3e2654ea6",
	"0x03aa8345aaf4bbc2ec790ac47fc1418edba38ec23c89989e45dc30549d945a02",
	"0x031808e5160a9c6eb4b1c693bedadc0d095556cc9d0bf388b01cfc985a5543db",
	"0x001f169cf0caa18109cfb6ee9c3ab13fbd4df68bd724d5378a7706cc6d24adb6",
	"0x045f93445b6d0dfba4cf75c75829bac9577d7dd181a3b26f5ce33da2d602f0d4",
	"0x00985537351d3852fb643f6b821c4f5bbf58d29dff86063557504904971a1158",
	"0x00e24f276083a1fc65f72741977a8b744d89d052693ce81ea38026f326a89d3c",
	"0x010d527b2864d0c9953108b1a90441d96c309f481dbe628b2a36207aca4e91d6",
	"0x04658b3e7bbec1614479e37a22113f8de02d683f37fd9806d447b0177513b8b7",
	"0x04b7e8e63f8dd018857d95eb658e22ba78b4f3b728d7deadd08629e9b390229a",
	"0x0329da8859a64bad7a3cb2004c6efce86495b71b74c6e3348aa44448cf3dfea7",
	"0x01207db1a7cfd3ee44ae995b24d1527514836bcac4f08e1934d210124b425b4c",
	"0x026e2c24c091005de66f530cb95fc159e4b08e93f8ab2801cbadadb5fad1b1de",
	"0x052cc5650abe0c693318c0779325774ff9bc30c102d226f461ddb4dc012a9f46",
	"0x03b2fcff01889757eccd6ea2c94a1e7a8233f45015da82a6a1f8cd23418af67c",
	"0x02068938b67e0898724be80d8fdf36e12ec95b14c944d2e6f5aa963c47020b76",
	"0x03c07b764f7fb998d05819234b158af6d7e346b2462656713f9b1fb7450b9c88",
	"0x021fd16b92621c659c94909b319525d7cbc021a2478e841e3485e38576b0949d",
	"0x00e6a374be5bc0bde41bdca34c93e4d92682921d7c3b48485a1ffc160b5a4679",
	"0x012e57b09f251d0fb3c60645b978a9c357dba277c23a208c9f5c8c2ef5e29342",
	"0x05f556a50d716b7d8728353c62fac78bde9c72f7bf2f18ee3e04dfcb78c36ccd",
	"0x009b2814982293b92f62f797922e4ef63baf925d0dcca9f6cd1b9b597ed892f8",
	"0x02840f0226abd403fe0b2c28d029887da92d027aeed7a6c75d58857c8291bd00",
	"0x05141d563645f8c52e82088e8c7b1948b99b3792dce406bb21dc22c594450840",
	"0x0160e5beddbcbbf50a0cd172b05427a2d25def57b771fb4847a23d044f886b38",
	"0x0562556303a4431b5158e4d3d0607682a8c5cc39de4ff8cdb2ae32c038a4e67e",
	"0x013ebfd0f4512a61952b56f676b58f8969379d7685eb4fca33278b775ff4e4e7",
	"0x01a08d7cc8cc94a1511f1577aff0e73176273a5a32dc6e8d1a7abf25edb66def",
	"0x03cb559d0408ffefcd942e07920e5e65c5592c60dbc85e2815032884439c40e9",
	"0x05f9cc0f39323abfdd3365bc03d6c8e60ca1ea6013fc8c59d824b4a0674fd827",
	"0x04db0d04629df941d67af7bfb4461bc21b13f506d637b71634fc6dd1134b6a8b",
	"0x01e7a640bec96c64ca3020e46f521b78c6cc4a68a723b99313e6a00677ee4031",
	"0x01e5c0b9e7012a9f3090d401da07bd278acac8ccb3c50c71a9297b0e85585f66",
	"0x0107097d5d86e410acb9f9e74c9e900a7ee0eacc774bd70ca253a9a34664b06c",
	"0x00333313628e55738bbfe230df688ca485689fd173f53c0524c83202d960f073",
	"0x05a87efd7cb1a6e89107c0ceab156006369726a31a7b7780833b0d7b8b398475",
	"0x003854a7d4b7495d8b9c79f8525c951bfe5df67a9a6d7fbe59a688f925d53dfe",
	"0x04786c3b578f433a4766f7bf43c0d576acb7009ff5753c743d04614b145d4a62",
	"0x022a8c0fa437a94270f12c422a316fd60afde24ade718ae738bd1849f82a3344",
	"0x022f3ac0a9f7041456314abc71ba4e73d69f2696bc2a694f89109ad51ac140f5",
	"0x004c716da2e1e3aaaf60c0ace35e816fcd3786474687a78667032c159372bfe3",
	"0x0129171dccded2ae12b112c4d63ef387fcae1fb9849fb6b77ecd19a0b4d1bbc9",
	"0x01d48a645a41b538d5f086145fc2e4157d6660e185b2cde91bfa06727cd76f4b",
	"0x040607d0710891d944fce5a5b37c3fd63b74f3ba154a936f6e38f38b9b94b54b",
	"0x024172d03eebd4652ef18973371c0c6bf192d731a503a724e3f40e5c0c6a09a7",
	"0x054fe3bb5a3bd4ac5a1acb0d3f50aaf0e7a50a108862c6120cd356a97925b360",
	"0x002b48b7bb3adf797dfeade710bae704b25235081b62c0cb6e7470c79808706e",
	"0x0122e96558ece09f6ba89182e38e7dd803dac71e291b06ef60af3c6db58ab8de",
	"0x041eec2e0e6ed9fc774dcedd162d58b5698523998624cbeb6017767df4bde6cd",
	"0x03d7778d88b24e19c22dc136653eb37878cacd0b3e2c63a91fee01778cb9909c",
	"0x0008287e2d03feb1a60ea9b6a8f11f76070d9e7e630bf2202d6954113f9fe3c2",
	"0x0320893784c1696e5d8256b8e7ea54a5c033a488ae1f37fd1cb7d84db88d06ce",
	"0x0205c818f4aeee6c5c2a5894cc4ba27e0636ce0abb2af0bbf6fa9860698836ca",
	"0x052c5c5d8012b3977c5596ec28af4a00befebbb40dd33659d7c41ef782732e5a",
	"0x01a075be080bd8369f0361801cca065497b82cb18e6b99f5495f32557bc142ed",
	"0x024e5bef79e4b92effb1a2e578cb45d388a9fe9c1e5eb37ac3df9ad9222f389d",
	"0x00c5b834ae57642d2dd852ec9fac88141f7843f9be2ac815dfaf2cbffb87e6ae",
	"0x043e849591938d2d50a584ef8dfe45cafeb27da019516a4281cbd96890a27263",
	"0x04d88c8626e584c779f58fc2afa196757ca4c1a146eaf38c07a25fad0edea846",
	"0x0461c307e79e8947d75b7b89a7b361240b20c01a322ca5059339e6af04d49f8f",
	"0x03177bf192bfc338bbd25544756aebd72f24dd0cfdeac1de59d3c2150f73b5f1",
	"0x0304f34d186ff0cd9acd5ec5961756fcfbd295e46560e6b169532fa50e41b374",
	"0x0301fddc6fc50412d91c62f8808a757a3d77ef3e5c23f604bc203cd0a4edee6f",
	"0x051196222ca2e39ae8e5ab800c8b52bcd088851d67dd3c1f6132682793fe4cc2",
	"0x02aa580900dfa556d3952ad12d7a7102886f17b1132c9484eb506bdd1b890aa1",
	"0x01590c5f2be7165a34bdddf980098dfd9d336b35d40b9f3136b7013ba16968f3",
	"0x0161f673459fa80f48953a7da0da2c492260c40451266eb04128e1ef6e496725",
	"0x0143e38699b0ae25f99fecf8de32ad95bcff6bb010e2a34c7eed6b3cbee35a11",
	"0x00cdd4103564f8abd332eb4b487fc369ed9fa58d365df92b6d1a87d01aeb552c",
	"0x03e26486058250c9c15e021dedaf4cb76a9de7153c85fb48bfdf875db69c615f",
	"0x005ebb30e445bddc41711060391b9c12bcefb828ab4b7296b28a62a35daa7c8c",
	"0x01ef5e32f7228798e10457daa3647ddc70a7dcb0567eab2540a31555a3c7251c",
	"0x0209e17ab4315e9aebe5387d95843a59a2862eea63da35df0bdc16c8bc180031",
	"0x045c02c7726bdadc7b28026aac23bc7108d50d39497bd3ef20823efa3e8aa1cf",
	"0x01213f6cb08faf5e438fe37825b6d96096587b794ff23ce180898d54e134f529",
	"0x02c661447f863b195f7174cc83b00bf91150e693e9b22cebc271d75b555764a1",
	"0x0563110ba87ffe5451c1412e1c8663ec120908ad4c5c203cb300022786ee37c9",
	"0x05cb08e5bfb885027687aac1e851102f87cd121b9279503b2dad65dd712042d4",
	"0x03657023d6df573ab1a663d59659e687530bf7a98a08e99205f5e1a27e01cfe3",
	"0x00c0194f6dd2693df110dc2d02102a25650700f47a7e7c716546d92b1c64d92b",
	"0x038471d27e3f2632d5f46feffdfcec68fa2dc0c59f379a78ecda111104e30d26",
	"0x0421ddd429a4a175577558d0949b08d9da9f776cc1b788d78316dbbab76267e6",
	"0x020a79016f1d81dd91307ee73d4d4346a9c9858919c87bd2138c462a1bb5a28a",
	"0x001ea42d505963c8deaaf41275c185ac3cf86b573bbfc4763d76f4e4bcccb033",
	"0x033eb19cf82317ee58e950d9b825842d4cc3b97567770b292f71585d52f17273",
	"0x02aeb8c1a766bfad6531fb4693edb1708a337c36121bebb31eb637783e2b586c",
	"0x03b2d8f700b7ac811085f8052e8d632be3584d46e14878a00e80f8c9e690206f",
	"0x04a986f328fb082c793d6a729aff0df5e3aad1861cc568159c7d77079da4193f",
	"0x04d9d40d6325ca5f02bd1835364df4c9fa968b01cbb986d30affd48b338a9851",
	"0x006bf417c45155c8d3cc8781a55f963587f5aefb4024794a6d1642201c986d18",
	"0x023ab6a14b328ca0ae40094eaffe96474e09b9e0b632e55f743dde041a05fda8",
	"0x02d813a0ba4da4213d42b87abf13df3f5ad1e306fd4204665eaf65c8354c2872",
	"0x026b6e0749644bfea276ed9300281e73272479ecdb7cd0259e37312fd90a9525",
	"0x037d4f6475965d5dc82dae2f03068751bcea5c13e3012c202508a9e61f55ee92",
	"0x05008e26851a9e78cc09fa25a9a3bd44cdba0f613997a0510263564b5e83980a",
	"0x01c8ec018f0b549c505206d4b6f94348c16ad6e8b23bd2f15290a8c417129baa",
	"0x0069b1dd8f3dd378e93117504d27c30d0157fc3c1034aeeb45c82b29996e3a07",
	"0x0193891f4c54eb430d28bd989034bd04b526726871f37891c07cfde328659337",
	"0x02e2f826e672027282cec477a1098cdfd72a5bbdb45a8a1f54e6dce3ab145c26",
	"0x02f6558ac7edbcfcb0568f302edce0cc3a399910d18309761a35bc1ba355971b",
	"0x05c7450764cff539415d204505848f8007e63d1e1597e6a0639d3f749da828f0",
	"0x027001c1ab1687762c8353fe26b646f32739d90f32eaf0af1a530698953ad3bd",
	"0x03877704c9e19c49fc4e48e5a4b105f88e47278226c30a8ab60171ec417473ec",
	"0x00cce3e7d2e362d7f3e4463074206964a242feb29375835f4eaa50abcdc038d8",
];
pub const MDS_ENTRIES: [[&str; 3]; 3] = [
	[
		"0x0222c2295933de604a294bbe7a6543c5d9ebd630053b7f9380a669de3d7eadf1",
		"0x03441d7052ea38fc9ecabd9a4557c8b1afe6cb2a0408562f451ab56da4afe86d",
		"0x02d9f47870148067d253a8669c5b8e295194ea699e75529f2b0ed514d097c66d",
	],
	[
		"0x036a615c3b5d832a4d6f6b3f88669d567a0af5b296ba6cb152c4da921eba51f2",
		"0x02c1e1bce367bbbe09358f3b92e61a45a1e42ce2fb07ddc5dacf8621c22eecbf",
		"0x0272f053b6c044a0577741fc91da95b72fb487b0f1ff01d9bfcbd4678a9626a7",
	],
	[
		"0x017855e36060ee0a09053c164d0ff3a0ab272b6c8a61881ef7e385abb6a9c198",
		"0x025f79d9fa0e66e16d8e1c9817709c76a6c5fc9c200af36892af3c1b43be1d6d",
		"0x0140865fd79f266cb7cc99a99fee9150af1e6421b869f813de90a4e5606bfd75",
	],
];
